
use core::cmp::min;

use crate::block_device::{
    BlockDevice,
    BlockError,
    SECTOR_SIZE,
};

const MAGIC: [u8; 8] = *b"BLOXFS01";
const VERSION: u32 = 1;

const SUPERBLOCK_LBA: u64 = 0;
const TABLE_START_LBA: u64 = 1;
const TABLE_SECTORS: usize = 8;
const DATA_START_LBA: u64 = 9;

const MAX_ENTRIES: usize = 64;
const NAME_LENGTH: usize = 40;
const ENTRY_SIZE: usize = 64;
const TABLE_SIZE: usize = TABLE_SECTORS * SECTOR_SIZE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsError {
    Disk(BlockError),
    InvalidFilesystem,
    InvalidName,
    NameTooLong,
    NotFound,
    AlreadyExists,
    DirectoryFull,
    NoSpace,
    BufferTooSmall,
    IsDirectory,
    IsFile,
    CorruptedMetadata,
}

impl From<BlockError> for FsError {
    fn from(value: BlockError) -> Self {
        Self::Disk(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EntryType {
    File = 1,
    Directory = 2,
}

#[derive(Clone, Copy)]
pub struct DirectoryEntry {
    pub name: [u8; NAME_LENGTH],
    pub name_len: u8,
    pub entry_type: u8,
    pub start_lba: u32,
    pub sector_count: u32,
    pub size: u32,
}

impl DirectoryEntry {
    pub const EMPTY: Self = Self {
        name: [0; NAME_LENGTH],
        name_len: 0,
        entry_type: 0,
        start_lba: 0,
        sector_count: 0,
        size: 0,
    };

    pub fn is_used(&self) -> bool {
        self.entry_type != 0
    }

    pub fn is_file(&self) -> bool {
        self.entry_type == EntryType::File as u8
    }

    pub fn is_directory(&self) -> bool {
        self.entry_type == EntryType::Directory as u8
    }

    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len as usize]
    }
}

pub struct BloxFs<D: BlockDevice> {
    device: D,
    entries: [DirectoryEntry; MAX_ENTRIES],
    total_sectors: u64,
}

impl<D: BlockDevice> BloxFs<D> {
    pub fn format(mut device: D) -> Result<Self, FsError> {
        let total_sectors = device.sector_count();

        if total_sectors <= DATA_START_LBA {
            return Err(FsError::NoSpace);
        }

        let mut superblock = [0u8; SECTOR_SIZE];

        superblock[0..8].copy_from_slice(&MAGIC);
        superblock[8..12].copy_from_slice(&VERSION.to_le_bytes());
        superblock[12..20].copy_from_slice(&total_sectors.to_le_bytes());
        superblock[20..24].copy_from_slice(&(MAX_ENTRIES as u32).to_le_bytes());

        device.write_sector(SUPERBLOCK_LBA, &superblock)?;

        let entries = [DirectoryEntry::EMPTY; MAX_ENTRIES];

        let mut fs = Self {
            device,
            entries,
            total_sectors,
        };

        fs.save_table()?;
        fs.device.flush()?;

        Ok(fs)
    }

    pub fn mount(mut device: D) -> Result<Self, FsError> {
        let mut superblock = [0u8; SECTOR_SIZE];

        device.read_sector(SUPERBLOCK_LBA, &mut superblock)?;

        if superblock[0..8] != MAGIC {
            return Err(FsError::InvalidFilesystem);
        }

        let version = u32::from_le_bytes([
            superblock[8],
            superblock[9],
            superblock[10],
            superblock[11],
        ]);

        if version != VERSION {
            return Err(FsError::InvalidFilesystem);
        }

        let total_sectors = u64::from_le_bytes([
            superblock[12],
            superblock[13],
            superblock[14],
            superblock[15],
            superblock[16],
            superblock[17],
            superblock[18],
            superblock[19],
        ]);

        if total_sectors > device.sector_count()
            || total_sectors <= DATA_START_LBA
        {
            return Err(FsError::InvalidFilesystem);
        }

        let mut table = [0u8; TABLE_SIZE];

        for sector in 0..TABLE_SECTORS {
            let mut buffer = [0u8; SECTOR_SIZE];

            device.read_sector(
                TABLE_START_LBA + sector as u64,
                &mut buffer,
            )?;

            let start = sector * SECTOR_SIZE;
            table[start..start + SECTOR_SIZE]
                .copy_from_slice(&buffer);
        }

        let mut entries = [DirectoryEntry::EMPTY; MAX_ENTRIES];

        for index in 0..MAX_ENTRIES {
            let offset = index * ENTRY_SIZE;
            let raw = &table[offset..offset + ENTRY_SIZE];

            if raw[41] == 0 {
                continue;
            }

            let name_len = raw[40];

            if name_len == 0 || name_len as usize > NAME_LENGTH {
                return Err(FsError::CorruptedMetadata);
            }

            let mut name = [0u8; NAME_LENGTH];
            name.copy_from_slice(&raw[0..NAME_LENGTH]);

            let start_lba = u32::from_le_bytes([
                raw[42], raw[43], raw[44], raw[45],
            ]);

            let sector_count = u32::from_le_bytes([
                raw[46], raw[47], raw[48], raw[49],
            ]);

            let size = u32::from_le_bytes([
                raw[50], raw[51], raw[52], raw[53],
            ]);

            let entry = DirectoryEntry {
                name,
                name_len,
                entry_type: raw[41],
                start_lba,
                sector_count,
                size,
            };

            if entry.is_file() {
                let end = (start_lba as u64)
                    .checked_add(sector_count as u64)
                    .ok_or(FsError::CorruptedMetadata)?;

                if (start_lba as u64) < DATA_START_LBA
                    || end > total_sectors
                    || size as u64 > sector_count as u64 * 512
                {
                    return Err(FsError::CorruptedMetadata);
                }
            } else if !entry.is_directory() {
                return Err(FsError::CorruptedMetadata);
            }

            entries[index] = entry;
        }

        Ok(Self {
            device,
            entries,
            total_sectors,
        })
    }

    fn validate_name(name: &str) -> Result<(), FsError> {
        let bytes = name.as_bytes();

        if bytes.is_empty()
            || bytes.len() > NAME_LENGTH
            || bytes.contains(&0)
            || bytes.contains(&b'/')
            || bytes.contains(&b'\\')
        {
            return Err(if bytes.len() > NAME_LENGTH {
                FsError::NameTooLong
            } else {
                FsError::InvalidName
            });
        }

        Ok(())
    }

    fn find_index(&self, name: &str) -> Option<usize> {
        self.entries.iter().position(|entry| {
            entry.is_used() && entry.name() == name.as_bytes()
        })
    }

    fn free_index(&self) -> Result<usize, FsError> {
        self.entries
            .iter()
            .position(|entry| !entry.is_used())
            .ok_or(FsError::DirectoryFull)
    }

    fn save_table(&mut self) -> Result<(), FsError> {
        let mut table = [0u8; TABLE_SIZE];

        for (index, entry) in self.entries.iter().enumerate() {
            let offset = index * ENTRY_SIZE;
            let raw = &mut table[offset..offset + ENTRY_SIZE];

            if !entry.is_used() {
                continue;
            }

            raw[0..NAME_LENGTH].copy_from_slice(&entry.name);
            raw[40] = entry.name_len;
            raw[41] = entry.entry_type;

            raw[42..46].copy_from_slice(&entry.start_lba.to_le_bytes());
            raw[46..50].copy_from_slice(&entry.sector_count.to_le_bytes());
            raw[50..54].copy_from_slice(&entry.size.to_le_bytes());
        }

        for sector in 0..TABLE_SECTORS {
            let start = sector * SECTOR_SIZE;
            let mut buffer = [0u8; SECTOR_SIZE];

            buffer.copy_from_slice(
                &table[start..start + SECTOR_SIZE],
            );

            self.device.write_sector(
                TABLE_START_LBA + sector as u64,
                &buffer,
            )?;
        }

        self.device.flush()?;
        Ok(())
    }

    fn extent_is_free(
        &self,
        start: u64,
        count: u64,
        ignored_index: Option<usize>,
    ) -> bool {
        let end = match start.checked_add(count) {
            Some(value) => value,
            None => return false,
        };

        if start < DATA_START_LBA || end > self.total_sectors {
            return false;
        }

        for (index, entry) in self.entries.iter().enumerate() {
            if !entry.is_used()
                || !entry.is_file()
                || Some(index) == ignored_index
                || entry.sector_count == 0
            {
                continue;
            }

            let other_start = entry.start_lba as u64;
            let other_end = other_start + entry.sector_count as u64;

            if start < other_end && other_start < end {
                return false;
            }
        }

        true
    }

    fn allocate(
        &self,
        count: u64,
        ignored_index: Option<usize>,
    ) -> Result<u64, FsError> {
        if count == 0 {
            return Ok(0);
        }

        let mut start = DATA_START_LBA;

        while start.checked_add(count).unwrap_or(u64::MAX)
            <= self.total_sectors
        {
            if self.extent_is_free(start, count, ignored_index) {
                return Ok(start);
            }

            start += 1;
        }

        Err(FsError::NoSpace)
    }

    pub fn create_file(&mut self, name: &str) -> Result<(), FsError> {
        Self::validate_name(name)?;

        if self.find_index(name).is_some() {
            return Err(FsError::AlreadyExists);
        }

        let index = self.free_index()?;
        let mut entry = DirectoryEntry::EMPTY;

        entry.name[..name.len()].copy_from_slice(name.as_bytes());
        entry.name_len = name.len() as u8;
        entry.entry_type = EntryType::File as u8;

        self.entries[index] = entry;
        self.save_table()
    }

    pub fn create_directory(
        &mut self,
        name: &str,
    ) -> Result<(), FsError> {
        Self::validate_name(name)?;

        if self.find_index(name).is_some() {
            return Err(FsError::AlreadyExists);
        }

        let index = self.free_index()?;
        let mut entry = DirectoryEntry::EMPTY;

        entry.name[..name.len()].copy_from_slice(name.as_bytes());
        entry.name_len = name.len() as u8;
        entry.entry_type = EntryType::Directory as u8;

        self.entries[index] = entry;
        self.save_table()
    }

    pub fn rename(
        &mut self,
        old_name: &str,
        new_name: &str,
    ) -> Result<(), FsError> {
        Self::validate_name(new_name)?;

        let index = self.find_index(old_name)
            .ok_or(FsError::NotFound)?;

        if self.find_index(new_name).is_some() {
            return Err(FsError::AlreadyExists);
        }

        self.entries[index].name = [0; NAME_LENGTH];
        self.entries[index].name[..new_name.len()]
            .copy_from_slice(new_name.as_bytes());

        self.entries[index].name_len = new_name.len() as u8;

        self.save_table()
    }

    pub fn delete(&mut self, name: &str) -> Result<(), FsError> {
        let index = self.find_index(name)
            .ok_or(FsError::NotFound)?;

        self.entries[index] = DirectoryEntry::EMPTY;
        self.save_table()
    }

    pub fn write_file(
        &mut self,
        name: &str,
        data: &[u8],
    ) -> Result<(), FsError> {
        let index = self.find_index(name)
            .ok_or(FsError::NotFound)?;

        if !self.entries[index].is_file() {
            return Err(FsError::IsDirectory);
        }

        if data.len() > u32::MAX as usize {
            return Err(FsError::NoSpace);
        }

        let sectors = (data.len() + SECTOR_SIZE - 1)
            / SECTOR_SIZE;

        let start = self.allocate(
            sectors as u64,
            Some(index),
        )?;

        for sector in 0..sectors {
            let mut buffer = [0u8; SECTOR_SIZE];
            let offset = sector * SECTOR_SIZE;
            let length = min(SECTOR_SIZE, data.len() - offset);

            buffer[..length]
                .copy_from_slice(&data[offset..offset + length]);

            self.device.write_sector(
                start + sector as u64,
                &buffer,
            )?;
        }

        self.device.flush()?;

        self.entries[index].start_lba = start as u32;
        self.entries[index].sector_count = sectors as u32;
        self.entries[index].size = data.len() as u32;

        self.save_table()
    }

    pub fn read_file(
        &mut self,
        name: &str,
        output: &mut [u8],
    ) -> Result<usize, FsError> {
        let index = self.find_index(name)
            .ok_or(FsError::NotFound)?;

        let entry = self.entries[index];

        if !entry.is_file() {
            return Err(FsError::IsDirectory);
        }

        let size = entry.size as usize;

        if output.len() < size {
            return Err(FsError::BufferTooSmall);
        }

        let mut remaining = size;

        for sector in 0..entry.sector_count as usize {
            if remaining == 0 {
                break;
            }

            let mut buffer = [0u8; SECTOR_SIZE];

            self.device.read_sector(
                entry.start_lba as u64 + sector as u64,
                &mut buffer,
            )?;

            let length = min(SECTOR_SIZE, remaining);
            let offset = sector * SECTOR_SIZE;

            output[offset..offset + length]
                .copy_from_slice(&buffer[..length]);

            remaining -= length;
        }

        Ok(size)
    }

    pub fn entries(&self) -> &[DirectoryEntry; MAX_ENTRIES] {
        &self.entries
    }

    pub fn total_sectors(&self) -> u64 {
        self.total_sectors
    }

    pub fn into_device(self) -> D {
        self.device
    }
}

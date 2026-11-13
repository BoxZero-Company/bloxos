
use crate::{FsError, MAX_FILES, MAX_NAME_LEN};
use crate::file::File;
use crate::metadata::Metadata;

pub struct Directory {
    name: [u8; MAX_NAME_LEN],
    name_len: usize,
    files: [Option<File>; MAX_FILES],
    pub metadata: Metadata,
}

impl Directory {
    pub fn new(name: &str) -> Result<Self, FsError> {
        let bytes = name.as_bytes();

        if bytes.is_empty() || bytes.contains(&b'/') || bytes.contains(&b'\\') {
            return Err(FsError::InvalidName);
        }

        if bytes.len() > MAX_NAME_LEN {
            return Err(FsError::NameTooLong);
        }

        let mut stored_name = [0u8; MAX_NAME_LEN];
        stored_name[..bytes.len()].copy_from_slice(bytes);

        Ok(Self {
            name: stored_name,
            name_len: bytes.len(),
            files: core::array::from_fn(|_| None),
            metadata: Metadata::new(true),
        })
    }

    pub fn root() -> Self {
        Self::new("/")
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len])
            .unwrap_or("")
    }

    pub fn create_file(&mut self, name: &str) -> Result<&mut File, FsError> {
        if self.find_file(name).is_some() {
            return Err(FsError::AlreadyExists);
        }

        let slot = self.files
            .iter()
            .position(Option::is_none)
            .ok_or(FsError::DirectoryFull)?;

        self.files[slot] = Some(File::new(name)?);

        self.files[slot]
            .as_mut()
            .ok_or(FsError::NotFound)
    }

    pub fn find_file(&self, name: &str) -> Option<&File> {
        self.files
            .iter()
            .filter_map(Option::as_ref)
            .find(|file| file.name() == name)
    }

    pub fn find_file_mut(&mut self, name: &str) -> Option<&mut File> {
        self.files
            .iter_mut()
            .filter_map(Option::as_mut)
            .find(|file| file.name() == name)
    }

    pub fn remove_file(&mut self, name: &str) -> Result<File, FsError> {
        let slot = self.files
            .iter()
            .position(|entry| {
                entry.as_ref().is_some_and(|file| file.name() == name)
            })
            .ok_or(FsError::NotFound)?;

        self.files[slot].take().ok_or(FsError::NotFound)
    }

    pub fn file_count(&self) -> usize {
        self.files.iter().filter(|entry| entry.is_some()).count()
    }

    pub fn files(&self) -> impl Iterator<Item = &File> {
        self.files.iter().filter_map(Option::as_ref)
    }
}

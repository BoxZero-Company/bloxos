
use crate::{FsError, MAX_FILE_SIZE, MAX_NAME_LEN};
use crate::metadata::Metadata;

pub struct File {
    name: [u8; MAX_NAME_LEN],
    name_len: usize,
    data: [u8; MAX_FILE_SIZE],
    len: usize,
    pub metadata: Metadata,
}

impl File {
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
            data: [0; MAX_FILE_SIZE],
            len: 0,
            metadata: Metadata::new(false),
        })
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len])
            .unwrap_or("")
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn write(&mut self, content: &[u8]) -> Result<(), FsError> {
        if content.len() > MAX_FILE_SIZE {
            return Err(FsError::FileTooLarge);
        }

        self.data[..content.len()].copy_from_slice(content);
        self.len = content.len();
        self.metadata.update_size(self.len);

        Ok(())
    }

    pub fn append(&mut self, content: &[u8]) -> Result<(), FsError> {
        let new_len = self.len
            .checked_add(content.len())
            .ok_or(FsError::FileTooLarge)?;

        if new_len > MAX_FILE_SIZE {
            return Err(FsError::FileTooLarge);
        }

        self.data[self.len..new_len].copy_from_slice(content);
        self.len = new_len;
        self.metadata.update_size(self.len);

        Ok(())
    }

    pub fn read(&self, output: &mut [u8]) -> Result<usize, FsError> {
        if output.len() < self.len {
            return Err(FsError::BufferTooSmall);
        }

        output[..self.len].copy_from_slice(&self.data[..self.len]);
        Ok(self.len)
    }

    pub fn contents(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn clear(&mut self) {
        self.data[..self.len].fill(0);
        self.len = 0;
        self.metadata.update_size(0);
    }

    pub fn touch(&mut self, timestamp: u64) {
        self.metadata.touch(timestamp);
    }
}


#![no_std]

pub mod block_device;
pub mod bloxfs;

pub use block_device::{BlockDevice, BlockError, SECTOR_SIZE};
pub use bloxfs::{BloxFs, DirectoryEntry, EntryType, FsError};

pub const FILESYSTEM_VERSION: &str = "0.2.0";

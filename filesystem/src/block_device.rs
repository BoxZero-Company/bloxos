
pub const SECTOR_SIZE: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockError {
    OutOfBounds,
    DeviceNotReady,
    ReadFailed,
    WriteFailed,
    FlushFailed,
}

pub trait BlockDevice {
    fn sector_count(&self) -> u64;

    fn read_sector(
        &mut self,
        lba: u64,
        buffer: &mut [u8; SECTOR_SIZE],
    ) -> Result<(), BlockError>;

    fn write_sector(
        &mut self,
        lba: u64,
        buffer: &[u8; SECTOR_SIZE],
    ) -> Result<(), BlockError>;

    fn flush(&mut self) -> Result<(), BlockError>;
}

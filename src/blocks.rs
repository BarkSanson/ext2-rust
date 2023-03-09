/// Normal File Systems are installed into a physical device,
/// such as an HDD, SDD, etc. Since this is a learning project,
/// having an actual device would be annoying to transport 
/// (although at this time, I'm writing this at home and I wouldn't 
/// have to bring the physical device anywhere) and would also mean
/// to buy an specific device for a single project. Thus, the way
/// this FS is going to be implemented is through a virtual device.
/// This virtual device will be a file, that will have the next internal
/// structure:
/// 
///     --------------------------------------------------------------------
///     |            |        |             |                              |
///     | Superblock | Bitmap | Inode array |             Data             |
///     |            |        |             |                              |
///     --------------------------------------------------------------------
/// 
/// As you can see, this is a bitmap and inode array implementation, since this
/// FS is inspired in the ext2 FS.

use std::{fs, fs::File, io::{self}, path};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use crate::util::bytes;

pub const BLOCKSIZE: usize = 2048;

pub struct BlockManager<T: Read + Write + Seek> {
    v_device: T,
}

impl BlockManager<File> {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let v_device: File;
        if path::Path::new(path).exists() {
            v_device = OpenOptions::new().write(true).read(true).open(path)?;
        } else {
            v_device = File::create(path)?;
        }
        v_device.set_permissions(fs::Permissions::from_mode(0o666))
            .expect("TODO: panic message");

        Ok(Self { v_device })
    }
}

impl<T: Read + Write + Seek> BlockManager<T> {
    fn from_in_memory(buffer: T) -> Self {
        Self { v_device: buffer }
    }

    pub fn write_block<E>(&mut self, block_number: usize, buffer: &E) -> Result<usize, io::Error> {
        let offset = BLOCKSIZE * block_number;

        let raw = bytes::to_u8_slice(buffer);

        self.v_device.seek(SeekFrom::Start(offset as u64))?;

        let bytes_written = self.v_device.write(raw)?;

        self.v_device.rewind()?;

        Ok(bytes_written)
    }

    pub fn read_block(&mut self, block_number: usize, buffer: &mut [u8; BLOCKSIZE]) -> Result<(), io::Error> {
        let offset = BLOCKSIZE * block_number;

        self.v_device.seek(SeekFrom::Start(offset as u64))?;

        self.v_device.read_exact(buffer)?;

        self.v_device.rewind()?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::blocks::BLOCKSIZE;
    use super::BlockManager;

    #[test]
    fn correct_bytes_are_written() {
        let buffer = Cursor::new(Vec::new());
        let mut block_manager = BlockManager::from_in_memory(buffer);
        let data: [u8; BLOCKSIZE] = [0xFF; BLOCKSIZE];
        let expected = BLOCKSIZE;
        let block_number = 0;

        let actual = block_manager.write_block(block_number, &data).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn correct_bytes_are_read() {
        let buffer = Cursor::new(Vec::new());
        let mut block_manager = BlockManager::from_in_memory(buffer);

        // Write different data in each consecutive block
        for i in 0..4 as usize {
            let data: [u8; BLOCKSIZE];
            if i % 2 == 0{
                data = [0xFF; BLOCKSIZE];
            } else {
                data = [0x00; BLOCKSIZE];
            }
            block_manager.write_block(i, &data).unwrap();
        }

        // Each consecutive block should have the same data written before
        for i in 0..4 as usize {
            let mut actual: [u8; BLOCKSIZE] = [0x66; BLOCKSIZE];
            block_manager.read_block(i, &mut actual).unwrap();

            if i % 2 == 0 {
                assert_eq!(actual, [0xFF; BLOCKSIZE]);
            } else {
                assert_eq!(actual, [0x00; BLOCKSIZE]);
            }
        }
    }
}
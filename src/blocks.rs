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

use std::{fs, fs::File, io::{self}, os::unix::prelude::FileExt};
use std::os::unix::fs::PermissionsExt;

pub const BLOCKSIZE: usize = 1024;

pub struct BlockManager {
    v_device: File,
    size: usize
}

impl BlockManager {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let v_device = File::create(path)?;
        v_device.set_permissions(fs::Permissions::from_mode(0o666))
            .expect("TODO: panic message");

        Ok (Self { v_device, size: 0 })
    }

    pub fn write_block(&mut self, block_number: u64, buffer: &[u8; BLOCKSIZE as usize]) -> Result<usize, io::Error> {
        let offset = BLOCKSIZE * block_number;

        let bytes_written = self.v_device.write_at(buffer, offset as u64)?;
        self.size += bytes_written;

        Ok(bytes_written)
    }

    pub fn read_block(&self, block_number: u64, buffer: &mut [u8; BLOCKSIZE as usize]) -> Result<(), io::Error> {
        let offset = BLOCKSIZE * block_number;

        self.v_device.read_exact_at(buffer, offset as u64)?;

        Ok(())
    }

}

// TODO: develop tests for this module
//#[cfg(test)]
//mod tests {
//    use std::fs;
//    use std::path::Path;
//    use crate::blocks::BlockManager;
//    use fs::remove_file;
//}
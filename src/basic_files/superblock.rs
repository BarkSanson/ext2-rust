use std::mem;
use mem::size_of;
use std::fs::File;
use crate::blocks::{BlockManager, BLOCKSIZE};
use super::inode::INODE_SIZE;

/// Superblock position in the block structure
const SUPERBLOCK_POSITION: usize = 0;

/// Superblock size in logical blocks
const SUPERBLOCK_SIZE: usize = 1;

/// The superblock is the structure that stores
/// the general information about the file system.
/// This information goes from start/end positions of
/// each field in the file system, to the amount of
/// free blocks of data, etc. Thus block will always
/// have a size of 1 logical block and will always
/// fill the first block of the system.
struct Superblock {
    bitmap_first_block_position: usize,
    bitmap_last_block_position: usize,
    inode_array_first_block_position: usize,
    inode_array_last_block_position: usize,
    data_first_block_position: usize,
    data_last_block_position: usize,

    /// Position of the root node ('/') relative
    /// to the inode array. This will typically
    /// contain a 0
    root_inode_position: usize,

    /// Position of the first free inode relative
    /// to the inode array. As nodes are filled by
    /// data, this value will dynamically change
    /// during execution
    first_free_inode_position: usize,

    /// Free blocks in the whole system, including
    /// the superblock itself.
    total_free_blocks: usize,

    total_free_inodes: usize,

    /// Total amount of blocks in the whole system
    total_blocks: usize,
    total_inodes: usize,

    // Padding to fill the whole logical block
    padding: [u8; BLOCKSIZE - size_of::<usize>()]
}

impl Superblock {
    pub fn new(
        manager: &BlockManager<File>,
        number_of_blocks: usize,
        number_of_inodes: usize) -> Self {
        let bitmap_first_block_position = SUPERBLOCK_POSITION + SUPERBLOCK_SIZE;
        let bitmap_last_block_position =
            bitmap_first_block_position + Superblock::calculate_bitmap_size(number_of_blocks) - 1;
        let inode_array_first_block_position = bitmap_last_block_position + 1;
        let inode_array_last_block_position =
            bitmap_first_block_position + Superblock::calculate_inode_array_size(number_of_inodes) - 1;
        let data_first_block_position = inode_array_last_block_position + 1;
        let data_last_block_position = number_of_blocks - 1;

        let superblock = Self {
            bitmap_first_block_position,
            bitmap_last_block_position,
            inode_array_first_block_position,
            inode_array_last_block_position,
            data_first_block_position,
            data_last_block_position,
            root_inode_position: 0,
            first_free_inode_position: 0,
            total_free_blocks: number_of_blocks,
            total_free_inodes: number_of_inodes,
            total_blocks: number_of_blocks,
            total_inodes: number_of_inodes,
            padding: [0xFF as u8; BLOCKSIZE - size_of::<usize>()],
        };

        // TODO: write superblock to the virtual device

    }

    /// Computes the needed size in logical blocks of the bitmap
    fn calculate_bitmap_size(number_of_blocks: usize) -> usize {
        let total_bytes = number_of_blocks / 8;
        let bitmap_size = total_bytes / BLOCKSIZE;

        // If modulo isn't 0, that means
        // it is necessary 1 more block
        if total_bytes % BLOCKSIZE != 0{
            return bitmap_size + 1;
        }

        bitmap_size
    }

    /// Computes the needed size in logical blocks of the inode array
    fn calculate_inode_array_size(number_of_inodes: usize) -> usize {
        let total_bytes_needed = number_of_inodes * INODE_SIZE;
        let inode_array_size = total_bytes_needed / BLOCKSIZE;

        // If modulo isn't 0, that means
        // it is necessary 1 more block
        if total_bytes_needed % BLOCKSIZE != 0{
            return inode_array_size + 1;
        }

        inode_array_size
    }
}

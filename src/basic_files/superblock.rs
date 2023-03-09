use std::{io, mem};
use mem::size_of;
use std::fs::File;
use crate::blocks::{BlockManager, BLOCKSIZE};
use crate::util::field_sizes_computing;


/// Superblock position in the block structure
const SUPERBLOCK_POSITION: usize = 0;

/// Superblock size in logical blocks
const SUPERBLOCK_SIZE: usize = 1;

/// Total number of fields of the superblock structure
const SUPERBLOCK_NUMBER_OF_FIELDS: usize = 13;

/// Padding needed to fill a whole block
const SUPERBLOCK_PADDING: usize = BLOCKSIZE - ((SUPERBLOCK_NUMBER_OF_FIELDS - 1) - size_of::<usize>());

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
    padding: [u8; SUPERBLOCK_PADDING]
}

impl Superblock {
    pub fn new(
        manager: &mut BlockManager<File>,
        number_of_blocks: usize,
        number_of_inodes: usize) -> Result<Self, io::Error> {
        let bitmap_first_block_position = SUPERBLOCK_POSITION + SUPERBLOCK_SIZE;
        let bitmap_last_block_position =
            bitmap_first_block_position +
                field_sizes_computing::calculate_bitmap_size(number_of_blocks) - 1;
        let inode_array_first_block_position = bitmap_last_block_position + 1;
        let inode_array_last_block_position =
            bitmap_first_block_position +
                field_sizes_computing::calculate_inode_array_size(number_of_inodes) - 1;
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
            padding: [0xFF as u8; SUPERBLOCK_PADDING],
        };

        // Write superblock to the virtual device
        manager.write_block(SUPERBLOCK_POSITION, &superblock)?;

        Ok(superblock)
    }

}
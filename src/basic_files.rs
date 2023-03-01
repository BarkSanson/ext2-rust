use std::mem::size_of;
use crate::blocks;
use crate::blocks::BLOCKSIZE;

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

enum InodeType {
    Free,
    Directory,
    File
}


use std::mem;
use std::mem::{size_of, size_of_val};
use std::time::SystemTime;
use crate::blocks;
use crate::blocks::BLOCKSIZE;

/// Size in bytes of an inode
pub const INODE_SIZE: usize = 256;

/// Amount of bytes to fulfill required alignment for [`Inode`]
/// in 64-bit architectures.
#[cfg(target_pointer_width = "64")]
const INODE_ALIGNMENT: usize = 6;

/// Amount of bytes to fulfill required alignment for [`Inode`]
/// in 32-bit architectures.
#[cfg(target_pointer_width = "32")]
const INODE_ALIGNMENT: usize = 2;

/// Amount of direct pointers that a inode can hold.
/// A direct pointer points directly to data blocks. This
/// means this pointer is actually the number of the block that
/// is holding the data.
/// ``` text
///     Inode                           Logical blocks
/// +-----------+
/// |  Direct   | ----------------> +-------------+
/// |  pointer  |                   |  Some data  |
/// |     0     |                   |    block    |
/// +-----------+                   +-------------+
/// |  Direct   | ----------------> +-------------+
/// |  pointer  |                   |   Another   |
/// |     1     |                   |    data     |
/// +-----------+                   |    block    |
///                                 +-------------+
///     ...            ...                 ...
/// ```
/// Data blocks pointed are not necessarily consecutive
const INODE_DIRECT_POINTERS: usize = 12;

/// Amount of indirect pointers that an inode can hold.
/// An indirect pointer points to a pointer block with [`N_POINTERS`]
/// pointers to data blocks or more pointer blocks.
/// ``` text
///     Inode                           Logical blocks
/// +-----------+
/// | Indirect  | --->  +-------------+  ---> +-------------+
/// |  pointer  |       |   Pointer   |       |  Data block |
/// |     0     |       |    block    |       +-------------+
/// |           |       +-------------+   ...      ...
/// +-----------+
/// |  Indirect | --->  +-------------+ ---> +---------------+
/// |  pointer  |       |   Pointer   |      |     Level 2   |
/// |     1     |       |    block    | ...  | pointer block |
/// +-----------+       +-------------+      +---------------+
/// |  Indirect |
/// |  pointer  | --->  +-------------+ ---> +---------------+ ---> +---------------+
/// |     2     |       |   Pointer   |      |    Level 2    |      |    Level 3    |
/// +-----------+       |    block    | ...  | pointer block | ...  | pointer block |
///                     +-------------+      +---------------+      +---------------+
/// ```
const INODE_INDIRECT_POINTERS: usize = 3;

/// Total number of pointers that a pointer block can
/// contain
const N_POINTERS: usize = BLOCKSIZE / size_of::<u32>();

enum InodeType {
    Free,
    Directory,
    File
}

impl InodeType {
    fn as_char(&self) -> char {
        match self {
            InodeType::Free => '\0',
            InodeType::Directory => 'd',
            InodeType::File => '-'
        }
    }
}

enum InodePermissions {
    None = 0b000,
    ExecuteOnly = 0b001,
    WriteOnly = 0b010,
    ReadOnly = 0b100,
    ReadExecute = 0b101,
    ReadWrite = 0b110,
    All = 0b111
}

struct Inode {
    inode_type: InodeType,
    permissions: InodePermissions,

    #[cfg(target_pointer_width = "64")]
    alignment: [u8; INODE_ALIGNMENT],

    /// Time of last data access
    a_time: SystemTime,

    /// Time of last modification of the data
    /// pointed by the inode
    m_time: SystemTime,

    /// Time of last modification of the inode
    c_time: SystemTime,

    /// Number of pointers to entries in a directory
    n_links: usize,

    size_in_logical_bytes: usize,
    num_of_occupied_blocks: usize,

    direct_pointers: [usize; INODE_DIRECT_POINTERS],
    indirect_pointers: [usize; INODE_INDIRECT_POINTERS],

    padding: [u8; INODE_SIZE
        - size_of::<InodeType>()
        - size_of::<InodePermissions>()
        - INODE_ALIGNMENT * size_of::<u8>()
        - 3 * size_of::<SystemTime>()
        - (INODE_DIRECT_POINTERS + INODE_INDIRECT_POINTERS + 3) * size_of::<usize>()
    ]
}

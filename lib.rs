#![allow(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub const SLOT_ID_BASE: u64 = 0x30_56_45_44_5F_4B_4C_42;
pub const SECTOR_SIZE:  u64 = 512;

/// Largest READ/WRITE request drivers accept: 1 MiB. A driver short of
/// DMA memory may accept less and answers EINVAL above its limit;
/// callers then retry in smaller pieces.
pub const MAX_XFER_SECTORS: u32 = 2048;

pub const OP_READ_SECTORS:  u32 = 0;
pub const OP_WRITE_SECTORS: u32 = 1;
pub const OP_FLUSH:         u32 = 2;
pub const OP_GEOMETRY:      u32 = 3;

/// Driver return values are system statuses from the error registry
/// (zigbone_abi::errors): 0 = OK, generic errors (EIO, ENODEV, EINVAL,
/// ENOSYS) where they fit, the `block` facility for device conditions.
pub use zigbone_abi::errors::{Error, OK, Status, block as errors};

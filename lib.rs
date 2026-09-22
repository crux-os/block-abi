#![allow(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub const SLOT_ID_BASE: u64 = 0x30_56_45_44_5F_4B_4C_42;
pub const SECTOR_SIZE:  u64 = 512;

pub const OP_READ_SECTORS:  u32 = 0;
pub const OP_WRITE_SECTORS: u32 = 1;
pub const OP_FLUSH:         u32 = 2;
pub const OP_GEOMETRY:      u32 = 3;

pub const E_OK:    i64 = 0;
pub const E_IO:    i64 = -5;
pub const E_NODEV: i64 = -19;
pub const E_INVAL: i64 = -22;
pub const E_NOSYS: i64 = -38;

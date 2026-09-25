#![allow(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub const SLOT_ID_BASE: u64 = 0x30_56_45_44_5F_4B_4C_42;
/// How many: BLK_DEV0 .. BLK_DEV0 + MAX_DEVICES - 1.
pub const MAX_DEVICES: u64 = 8;
pub const SECTOR_SIZE:  u64 = 512;

/// Largest READ/WRITE request drivers accept: 1 MiB. A driver short of
/// DMA memory may accept less and answers EINVAL above its limit;
/// callers then retry in smaller pieces.
pub const MAX_XFER_SECTORS: u32 = 2048;

pub const OP_READ_SECTORS:  u32 = 0;
pub const OP_WRITE_SECTORS: u32 = 1;
pub const OP_FLUSH:         u32 = 2;
pub const OP_GEOMETRY:      u32 = 3;

// ── Asynchronous requests ────────────────────────────────────────────
// A client opens a queue, submits batches of requests and reaps their
// completions later; many requests are in flight at once and the device
// is kicked once per batch. Buffers must stay valid (and untouched) until
// their completion is reaped: the device transfers straight into them.
// Requests complete in any order -- submit a FLUSH only after the writes
// it covers have completed.
//
//   OP_QUEUE_OPEN  ()                           -> queue id (>= 0)
//   OP_SUBMIT      (queue, *const Request, n)   -> requests taken (> 0);
//                  EAGAIN when the queue is full of unreaped completions
//   OP_REAP        (queue, *mut Completion, max | min << 32, timeout_ms)
//                  -> completions copied, waiting for `min` (ETIMEDOUT;
//                     timeout 0 = no limit)
//   OP_QUEUE_CLOSE (queue)                      -> waits for its requests
//
// A request the driver can't run asynchronously completes with E2BIG
// (buffer too scattered for one transfer); run it synchronously instead.
// ENOSYS from OP_QUEUE_OPEN: the driver has no asynchronous path.
pub const OP_QUEUE_OPEN:  u32 = 4;
pub const OP_SUBMIT:      u32 = 5;
pub const OP_REAP:        u32 = 6;
pub const OP_QUEUE_CLOSE: u32 = 7;

/// Requests submitted and not yet reaped, per queue.
pub const QUEUE_DEPTH: u32 = 64;

/// `op`: OP_READ_SECTORS, OP_WRITE_SECTORS or OP_FLUSH.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Request {
    pub op: u32,
    pub count: u32,
    pub lba: u64,
    pub buf: u64,
    pub tag: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Completion {
    pub tag: u64,
    pub status: i64,
}

/// Driver return values are system statuses from the error registry
/// (zigbone_abi::errors): 0 = OK, generic errors (EIO, ENODEV, EINVAL,
/// ENOSYS) where they fit, the `block` facility for device conditions.
pub use zigbone_abi::errors::{Error, OK, Status, block as errors};

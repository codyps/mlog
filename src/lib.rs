//! mlog defines a circular log structure for reliable logging on microcontrollers
//!
//!
//! To use `mlog`, one:
//!
//! 1. defines a Log, which is a area of memory which persists across resets
//! 2. define consumers of this Log, which might be things like serialization to flash memory or
//!    sending over a network interface
//! 3. 
#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]
use std::convert::From;

/// The `Log` is an area of memory that is persisted across some (but not necessarily all) events.
///
/// For example, it might persist across soft resets, but be cleared by a power-on-reset (ie: power
/// loss). A region of RAM that can be preserved across resets can be a good choice for the `Log`
/// location.
///
/// Entries in the `Log` logically have sequence numbers that allow totally ordering all entries.
/// Note: we say "logically" here because to improve storage efficiency, these "global" sequence
/// numbers might not be stored next to each entry in memory, and instead might be embedded in the
/// Log structure itself.
///
/// Physically, the Log structure has the following layout:
/// ```
/// struct LogLayout {
///     /// LOG_MAGIC_V1
///     magic: u32,
///     /// Number of bytes considered part of the log
///     /// This is included so that when the log size changes we can correctly invalidate our
///     /// header
///     length: u32,
///     /// Randomly generated on Log creation. Allows determining if external `Cursors` (which may
///     /// live past the lifetime of the `Log` by being on remote systems) are still valid.
///     /// On generation, the lower 32-bits are zeroed. When the log completes a "cycle" the epoch
///     /// is incremented by 1.
///     epoch: u64,
///
///     /// Note on `head` and `tail`: the majority of the time, the log is expected to be full.
///     /// When the log is full these will both be modified by the consumers, and will be directly
///     /// related to one another. It may be useful to consider another mechanism for encoding
///     /// this data given the nature of the Log is somewhat different than a normal circular
///     /// queue.
///
///     /// Location where the next entry will be written
///     head: u16,
///     /// Location where the last entry is located
///     tail: u16,
/// }
/// ```
#[derive(Debug)]
pub struct Log<'a> {
    data: &'a [u8]
}

impl<'a> From<&'a [u8]> for Log<'a> {
    fn from(data: &'a [u8]) -> Self {
        Log {
            data
        }
    }
}

impl<'a> Log<'a> {

    /// Obtain a cursor that will start reading from the oldest entry in the log
    pub fn cursor_from_start(&self) -> Cursor {
        todo!() 
    }

    /// Obtain a cursor that will start reading from the next entry that is added to the log
    ///
    /// In other words, nothing can be read from this cursor until another entry is inserted into
    /// the log.
    pub fn cursor_from_end(&self) -> Cursor {
        todo!()
    }

    /// Read the entry immediately after `cursor`, and return that entry and a new cursor.
    pub fn read_at(&self, _cursor: &Cursor) -> Option<(Entry<'a>, Cursor)> {
        todo!();
    }
}

/// A `Cursor` is a location within the [`Log`] which allows (when combined with the [`Log`]),
/// reading subseqeunt log entries.
#[derive(Debug)]
pub struct Cursor {

}

/// 
#[derive(Debug)]
pub struct Entry<'a> {
    data: &'a [u8],
}

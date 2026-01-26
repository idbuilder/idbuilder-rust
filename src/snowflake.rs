//! Local snowflake ID generator.
//!
//! This module provides a thread-safe snowflake ID generator that can be used
//! after fetching the configuration from the server.

use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Error, Result};

/// Thread-safe local snowflake ID generator.
///
/// Generates unique 64-bit IDs composed of:
/// - Timestamp (milliseconds since epoch)
/// - Worker ID (assigned by the server)
/// - Sequence number (per-millisecond counter)
///
/// # Example
///
/// ```
/// use idbuilder::SnowflakeGenerator;
///
/// let generator = SnowflakeGenerator::new(1, 1704067200000, 10, 12);
/// let id = generator.next_id().unwrap();
/// println!("Generated ID: {}", id);
/// ```
#[derive(Debug)]
pub struct SnowflakeGenerator {
    /// Custom epoch timestamp in milliseconds.
    epoch: i64,

    /// Allocated worker ID.
    worker_id: u32,

    /// Number of bits for worker ID.
    worker_bits: u8,

    /// Number of bits for sequence number.
    sequence_bits: u8,

    /// Maximum sequence value before overflow.
    max_sequence: i64,

    /// Current sequence number (atomic for thread safety).
    sequence: AtomicI64,

    /// Last timestamp when an ID was generated.
    last_timestamp: AtomicI64,
}

impl SnowflakeGenerator {
    /// Create a new snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `worker_id` - The worker ID assigned by the server
    /// * `epoch` - Custom epoch timestamp in milliseconds
    /// * `worker_bits` - Number of bits allocated for worker ID
    /// * `sequence_bits` - Number of bits allocated for sequence number
    #[must_use]
    pub const fn new(worker_id: u32, epoch: i64, worker_bits: u8, sequence_bits: u8) -> Self {
        let max_sequence = (1_i64 << sequence_bits) - 1;

        Self {
            epoch,
            worker_id,
            worker_bits,
            sequence_bits,
            max_sequence,
            sequence: AtomicI64::new(0),
            last_timestamp: AtomicI64::new(0),
        }
    }

    /// Generate the next unique ID.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The system clock moved backwards
    /// - The sequence overflows within a single millisecond (will wait for next ms)
    ///
    /// # Thread Safety
    ///
    /// This method is safe to call from multiple threads concurrently.
    pub fn next_id(&self) -> Result<i64> {
        loop {
            let timestamp = Self::current_timestamp()?;
            let last_ts = self.last_timestamp.load(Ordering::Acquire);

            if timestamp < last_ts {
                return Err(Error::ClockMovedBackwards);
            }

            if timestamp == last_ts {
                // Same millisecond, increment sequence
                let seq = self.sequence.fetch_add(1, Ordering::AcqRel);
                if seq <= self.max_sequence {
                    return Ok(self.compose_id(timestamp, seq));
                }

                // Sequence overflow, wait for next millisecond
                Self::wait_next_millis(timestamp)?;
                continue;
            }

            // New millisecond, try to update timestamp and reset sequence
            if self
                .last_timestamp
                .compare_exchange(last_ts, timestamp, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                self.sequence.store(1, Ordering::Release);
                return Ok(self.compose_id(timestamp, 0));
            }

            // Another thread updated the timestamp, retry
        }
    }

    /// Generate multiple IDs at once.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of IDs to generate
    ///
    /// # Errors
    ///
    /// Returns an error if any ID generation fails.
    pub fn next_ids(&self, count: usize) -> Result<Vec<i64>> {
        let mut ids = Vec::with_capacity(count);
        for _ in 0..count {
            ids.push(self.next_id()?);
        }
        Ok(ids)
    }

    /// Get the worker ID.
    #[must_use]
    pub const fn worker_id(&self) -> u32 {
        self.worker_id
    }

    /// Get the epoch.
    #[must_use]
    pub const fn epoch(&self) -> i64 {
        self.epoch
    }

    /// Decompose an ID into its components.
    ///
    /// Returns a tuple of (timestamp, worker ID, sequence).
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn decompose(&self, id: i64) -> (i64, u32, i64) {
        let worker_shift = self.sequence_bits;
        let ts_shift = self.worker_bits + self.sequence_bits;

        let sequence_mask = (1_i64 << self.sequence_bits) - 1;
        let worker_mask = (1_i64 << self.worker_bits) - 1;

        let sequence = id & sequence_mask;
        let worker_id = ((id >> worker_shift) & worker_mask) as u32;
        let timestamp = (id >> ts_shift) + self.epoch;

        (timestamp, worker_id, sequence)
    }

    fn compose_id(&self, timestamp: i64, sequence: i64) -> i64 {
        let ts_shift = u32::from(self.worker_bits) + u32::from(self.sequence_bits);
        let worker_shift = u32::from(self.sequence_bits);

        ((timestamp - self.epoch) << ts_shift)
            | (i64::from(self.worker_id) << worker_shift)
            | sequence
    }

    #[allow(clippy::cast_possible_truncation)]
    fn current_timestamp() -> Result<i64> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .map_err(|_| Error::ClockMovedBackwards)
    }

    fn wait_next_millis(current_ts: i64) -> Result<i64> {
        loop {
            let ts = Self::current_timestamp()?;
            if ts > current_ts {
                return Ok(ts);
            }
            std::hint::spin_loop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_single_id() {
        let gen = SnowflakeGenerator::new(1, 1_704_067_200_000, 10, 12);
        let id = gen.next_id().unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_generate_multiple_ids() {
        let gen = SnowflakeGenerator::new(1, 1_704_067_200_000, 10, 12);
        let ids = gen.next_ids(100).unwrap();

        assert_eq!(ids.len(), 100);

        // All IDs should be unique
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 100);

        // IDs should be monotonically increasing
        for window in ids.windows(2) {
            assert!(window[1] > window[0]);
        }
    }

    #[test]
    fn test_decompose_id() {
        let gen = SnowflakeGenerator::new(42, 1_704_067_200_000, 10, 12);
        let id = gen.next_id().unwrap();

        let (timestamp, worker_id, sequence) = gen.decompose(id);

        assert_eq!(worker_id, 42);
        assert!(timestamp > 1_704_067_200_000);
        assert!(sequence < (1 << 12));
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let gen = Arc::new(SnowflakeGenerator::new(1, 1_704_067_200_000, 10, 12));
        let mut handles = vec![];

        for _ in 0..4 {
            let gen = Arc::clone(&gen);
            handles.push(thread::spawn(move || gen.next_ids(1000).unwrap()));
        }

        let mut all_ids = vec![];
        for handle in handles {
            all_ids.extend(handle.join().unwrap());
        }

        // All IDs should be unique
        let count = all_ids.len();
        all_ids.sort_unstable();
        all_ids.dedup();
        assert_eq!(all_ids.len(), count);
    }

    #[test]
    fn test_worker_id_accessor() {
        let gen = SnowflakeGenerator::new(123, 1_704_067_200_000, 10, 12);
        assert_eq!(gen.worker_id(), 123);
    }

    #[test]
    fn test_epoch_accessor() {
        let gen = SnowflakeGenerator::new(1, 1_704_067_200_000, 10, 12);
        assert_eq!(gen.epoch(), 1_704_067_200_000);
    }
}

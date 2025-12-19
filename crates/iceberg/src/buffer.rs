// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Event batching buffer for collecting events before writing to Iceberg.
//!
//! This module provides a thread-safe ring buffer that collects events with
//! configurable batch size and flush interval. Events are buffered in memory
//! and flushed either when the batch size is reached or the flush interval expires.

use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use thiserror::Error;

/// Errors that can occur during buffer operations.
#[derive(Error, Debug)]
pub enum BufferError {
    /// Buffer is empty when attempting to flush.
    #[error("Buffer is empty")]
    BufferEmpty,

    /// Buffer operation failed.
    #[error("Buffer operation failed: {0}")]
    OperationFailed(String),
}

/// Configuration for the event buffer.
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Maximum number of events to buffer before automatic flush.
    pub max_batch_size: usize,
    /// Maximum duration to wait before automatic flush.
    pub flush_interval: Duration,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 10_000,
            flush_interval: Duration::from_secs(60),
        }
    }
}

impl BufferConfig {
    /// Creates a new buffer configuration.
    #[must_use]
    pub fn new(max_batch_size: usize, flush_interval: Duration) -> Self {
        Self {
            max_batch_size,
            flush_interval,
        }
    }
}

/// Internal state of the event buffer.
struct BufferState<T> {
    /// Ring buffer storage.
    events: Vec<T>,
    /// Current write position in the ring buffer.
    write_pos: usize,
    /// Number of events currently in the buffer.
    count: usize,
    /// Timestamp of the last flush operation.
    last_flush: Instant,
}

impl<T> BufferState<T> {
    fn new(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            write_pos: 0,
            count: 0,
            last_flush: Instant::now(),
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self, max_size: usize) -> bool {
        self.count >= max_size
    }
}

/// Thread-safe ring buffer for batching events.
///
/// The buffer collects events and provides both size-based and time-based
/// flushing mechanisms. When the buffer reaches `max_batch_size` or when
/// `flush_interval` elapses, the events should be flushed.
///
/// # Type Parameters
///
/// * `T` - The event type to buffer. Must implement `Clone + Send`.
///
/// # Example
///
/// ```no_run
/// use std::time::Duration;
/// use nautilus_iceberg::buffer::{EventBuffer, BufferConfig};
/// use nautilus_iceberg::types::OrderbookL3Event;
///
/// let config = BufferConfig::new(1000, Duration::from_secs(30));
/// let buffer = EventBuffer::<OrderbookL3Event>::new(config);
///
/// // Push events (non-blocking)
/// // buffer.push(event);
///
/// // Check if flush is needed
/// if buffer.should_flush() {
///     let events = buffer.flush().unwrap();
///     // Write events to storage
/// }
/// ```
pub struct EventBuffer<T: Clone + Send> {
    config: BufferConfig,
    state: Arc<Mutex<BufferState<T>>>,
}

impl<T: Clone + Send> EventBuffer<T> {
    /// Creates a new event buffer with the specified configuration.
    #[must_use]
    pub fn new(config: BufferConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(BufferState::new(config.max_batch_size))),
            config,
        }
    }

    /// Pushes an event into the buffer.
    ///
    /// This is a non-blocking operation. If the buffer is full, the oldest
    /// event will be dropped to make room for the new one (ring buffer behavior).
    ///
    /// # Performance
    ///
    /// This operation is optimized for high throughput (100K+ events/sec).
    pub fn push(&self, event: T) {
        let mut state = self.state.lock();

        // If buffer is at capacity, use ring buffer semantics
        if state.events.len() < self.config.max_batch_size {
            state.events.push(event);
            state.count = state.events.len();
            state.write_pos = state.count % self.config.max_batch_size;
        } else {
            // Overwrite oldest event (ring buffer full)
            let write_pos = state.write_pos;
            state.events[write_pos] = event;
            state.write_pos = (write_pos + 1) % self.config.max_batch_size;
            state.count = self.config.max_batch_size;
        }
    }

    /// Checks if the buffer should be flushed based on size or time.
    ///
    /// Returns `true` if either:
    /// - The buffer has reached `max_batch_size`
    /// - The `flush_interval` has elapsed since the last flush
    #[must_use]
    pub fn should_flush(&self) -> bool {
        let state = self.state.lock();

        if state.is_empty() {
            return false;
        }

        state.is_full(self.config.max_batch_size)
            || state.last_flush.elapsed() >= self.config.flush_interval
    }

    /// Flushes the buffer and returns all buffered events.
    ///
    /// This operation clears the buffer and resets the flush timer.
    ///
    /// # Errors
    ///
    /// Returns `BufferError::BufferEmpty` if there are no events to flush.
    pub fn flush(&self) -> Result<Vec<T>, BufferError> {
        let mut state = self.state.lock();

        if state.is_empty() {
            return Err(BufferError::BufferEmpty);
        }

        // Take all events and reset the buffer
        let events = state.events.drain(..).collect();
        state.write_pos = 0;
        state.count = 0;
        state.last_flush = Instant::now();

        Ok(events)
    }

    /// Returns the current number of events in the buffer.
    #[must_use]
    pub fn len(&self) -> usize {
        self.state.lock().count
    }

    /// Returns `true` if the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.state.lock().is_empty()
    }

    /// Returns the maximum batch size.
    #[must_use]
    pub fn max_batch_size(&self) -> usize {
        self.config.max_batch_size
    }

    /// Returns the flush interval.
    #[must_use]
    pub fn flush_interval(&self) -> Duration {
        self.config.flush_interval
    }

    /// Forces a flush if there are any events, regardless of time or size.
    ///
    /// Returns `None` if the buffer is empty.
    #[must_use]
    pub fn try_flush(&self) -> Option<Vec<T>> {
        self.flush().ok()
    }
}

impl<T: Clone + Send> Clone for EventBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestEvent {
        id: u64,
        data: String,
    }

    #[test]
    fn test_buffer_creation() {
        let config = BufferConfig::new(100, Duration::from_secs(10));
        let buffer = EventBuffer::<TestEvent>::new(config);

        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert_eq!(buffer.max_batch_size(), 100);
        assert_eq!(buffer.flush_interval(), Duration::from_secs(10));
    }

    #[test]
    fn test_push_and_len() {
        let config = BufferConfig::new(100, Duration::from_secs(10));
        let buffer = EventBuffer::new(config);

        for i in 0..10 {
            buffer.push(TestEvent {
                id: i,
                data: format!("event_{}", i),
            });
        }

        assert_eq!(buffer.len(), 10);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_flush_size_based() {
        let config = BufferConfig::new(5, Duration::from_secs(100));
        let buffer = EventBuffer::new(config);

        // Push 5 events
        for i in 0..5 {
            buffer.push(TestEvent {
                id: i,
                data: format!("event_{}", i),
            });
        }

        assert!(buffer.should_flush());

        let events = buffer.flush().unwrap();
        assert_eq!(events.len(), 5);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_flush_time_based() {
        let config = BufferConfig::new(100, Duration::from_millis(10));
        let buffer = EventBuffer::new(config);

        buffer.push(TestEvent {
            id: 1,
            data: "event".to_string(),
        });

        // Should not flush immediately
        assert!(!buffer.should_flush());

        // Wait for flush interval
        std::thread::sleep(Duration::from_millis(15));

        // Should flush now
        assert!(buffer.should_flush());

        let events = buffer.flush().unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_flush_empty_buffer() {
        let config = BufferConfig::default();
        let buffer = EventBuffer::<TestEvent>::new(config);

        let result = buffer.flush();
        assert!(matches!(result, Err(BufferError::BufferEmpty)));
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let config = BufferConfig::new(5, Duration::from_secs(100));
        let buffer = EventBuffer::new(config);

        // Push 10 events into a buffer with capacity 5
        for i in 0..10 {
            buffer.push(TestEvent {
                id: i,
                data: format!("event_{}", i),
            });
        }

        // Buffer should only contain last 5 events (ring buffer behavior)
        assert_eq!(buffer.len(), 5);

        let events = buffer.flush().unwrap();
        assert_eq!(events.len(), 5);

        // Should contain events 5-9
        assert_eq!(events[0].id, 5);
        assert_eq!(events[4].id, 9);
    }

    #[test]
    fn test_try_flush() {
        let config = BufferConfig::default();
        let buffer = EventBuffer::new(config);

        // Empty buffer
        assert!(buffer.try_flush().is_none());

        // Add event
        buffer.push(TestEvent {
            id: 1,
            data: "test".to_string(),
        });

        // Should return events
        let events = buffer.try_flush().unwrap();
        assert_eq!(events.len(), 1);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_clone() {
        let config = BufferConfig::new(100, Duration::from_secs(10));
        let buffer1 = EventBuffer::new(config);

        buffer1.push(TestEvent {
            id: 1,
            data: "test".to_string(),
        });

        let buffer2 = buffer1.clone();

        // Both buffers should see the same state
        assert_eq!(buffer1.len(), 1);
        assert_eq!(buffer2.len(), 1);

        // Push to one buffer
        buffer2.push(TestEvent {
            id: 2,
            data: "test2".to_string(),
        });

        // Both should see the update (shared state)
        assert_eq!(buffer1.len(), 2);
        assert_eq!(buffer2.len(), 2);
    }

    #[test]
    fn test_should_not_flush_empty() {
        let config = BufferConfig::new(10, Duration::from_secs(1));
        let buffer = EventBuffer::<TestEvent>::new(config);

        assert!(!buffer.should_flush());
    }

    #[test]
    fn test_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = BufferConfig::new(1000, Duration::from_secs(10));
        let buffer = Arc::new(EventBuffer::new(config));

        let mut handles = vec![];

        // Spawn 10 threads, each pushing 100 events
        for thread_id in 0..10 {
            let buffer_clone = Arc::clone(&buffer);
            let handle = thread::spawn(move || {
                for i in 0..100 {
                    buffer_clone.push(TestEvent {
                        id: thread_id * 100 + i,
                        data: format!("thread_{}_event_{}", thread_id, i),
                    });
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have 1000 events (exactly at capacity)
        assert_eq!(buffer.len(), 1000);
        assert!(buffer.should_flush());
    }
}

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

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a timestamp nonce in microseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeNonce(u64);

impl TimeNonce {
    /// Creates a new TimeNonce with the current time in microseconds.
    pub fn now() -> Self {
        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64;
        Self(micros)
    }

    /// Creates a TimeNonce from microseconds.
    pub fn from_micros(micros: u64) -> Self {
        Self(micros)
    }

    /// Gets the nonce value in microseconds.
    pub fn as_micros(&self) -> u64 {
        self.0
    }

    /// Gets the nonce value in milliseconds.
    pub fn as_millis(&self) -> u64 {
        self.0 / 1000
    }
}

impl From<u64> for TimeNonce {
    fn from(micros: u64) -> Self {
        Self(micros)
    }
}

impl From<TimeNonce> for u64 {
    fn from(nonce: TimeNonce) -> Self {
        nonce.0
    }
}

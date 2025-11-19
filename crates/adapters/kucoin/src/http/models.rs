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

//! Kucoin HTTP response models.

use serde::{Deserialize, Serialize};

/// Standard Kucoin API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KucoinResponse<T> {
    /// Response code ("200000" for success)
    pub code: String,
    /// Response data
    pub data: T,
}

/// Server time response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KucoinServerTime {
    /// Server time in milliseconds
    pub time: u64,
}

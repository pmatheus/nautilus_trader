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

//! WebSocket client implementation for the Bitget WebSocket API.
//!
//! This module provides the WebSocket client for subscribing to
//! public and private data streams from Bitget.

pub mod client;
pub mod enums;
pub mod error;
pub mod messages;

pub use client::BitgetWebSocketClient;
pub use enums::*;
pub use error::{BitgetWsError, BitgetWsResult};
pub use messages::*;

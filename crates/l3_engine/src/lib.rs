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

//! L3 orderbook reconstruction engine.
//!
//! This crate provides functionality to reconstruct Level 3 (order-by-order) orderbook state
//! from Level 2 (aggregated price level) market data deltas. It synthesizes order IDs,
//! correlates L2 delta changes to synthetic L3 orders, and emits L3 delta events suitable
//! for downstream orderbook processing.
//!
//! # Architecture
//!
//! The L3 engine consists of several key components:
//!
//! - **Order ID Generator**: Synthesizes unique order IDs for reconstructed orders
//! - **Correlator**: Matches L2 delta changes to existing synthetic L3 orders
//! - **State Manager**: Maintains local orderbook state for correlation
//! - **Emitter**: Produces L3 delta events from correlation results
//! - **Engine**: Orchestrates the reconstruction pipeline
//! - **Exchange Adapters**: Handle exchange-specific quirks and conventions

pub mod correlator;
pub mod emitter;
pub mod engine;
pub mod exchange;
pub mod order_id;
pub mod state;

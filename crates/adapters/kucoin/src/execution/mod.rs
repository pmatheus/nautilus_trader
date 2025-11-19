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

//! Kucoin execution client module.

use nautilus_model::identifiers::AccountId;

/// Kucoin execution client for order management.
///
/// Stub implementation - to be completed with full execution handling.
#[derive(Clone)]
pub struct KucoinExecutionClient {
    account_id: AccountId,
}

impl KucoinExecutionClient {
    /// Creates a new [`KucoinExecutionClient`] instance.
    #[must_use]
    pub fn new(account_id: AccountId) -> Self {
        Self { account_id }
    }
}

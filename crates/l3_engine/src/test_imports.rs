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

//! Test module to verify nautilus_model imports work correctly.
//!
//! This module demonstrates that the l3_engine crate can successfully import
//! and use types from nautilus_model, which is a critical dependency.

#[cfg(test)]
mod tests {
    // Verify we can import from nautilus_model
    use nautilus_model::data::delta::OrderBookDelta;
    use nautilus_model::data::trade::TradeTick;
    use nautilus_model::identifiers::InstrumentId;
    use nautilus_model::orderbook::OrderBook;

    #[test]
    fn test_nautilus_model_types_available() {
        // This test verifies that we can reference nautilus_model types
        // The actual usage will be in the implementation modules
        assert!(true, "nautilus_model types are accessible");
    }
}

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

//! Parsing utilities for Bitget API responses and WebSocket messages.

use anyhow::Result;
use nautilus_model::identifiers::InstrumentId;
use ustr::Ustr;

/// Parses a Bitget symbol into a Nautilus [`InstrumentId`].
///
/// # Arguments
///
/// * `symbol` - The Bitget symbol (e.g., "BTCUSDT")
/// * `venue` - The venue name
///
/// # Returns
///
/// A [`InstrumentId`] constructed from the symbol and venue
pub fn parse_instrument_id(symbol: &str, venue: &str) -> Result<InstrumentId> {
    let instrument_id = InstrumentId::from(format!("{}.{}", symbol, venue).as_str());
    Ok(instrument_id)
}

/// Converts a Nautilus [`InstrumentId`] to a Bitget symbol.
///
/// # Arguments
///
/// * `instrument_id` - The Nautilus instrument ID
///
/// # Returns
///
/// The Bitget symbol string
pub fn instrument_id_to_symbol(instrument_id: &InstrumentId) -> Ustr {
    instrument_id.symbol.as_str().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instrument_id() {
        let result = parse_instrument_id("BTCUSDT", "BITGET");
        assert!(result.is_ok());
        let instrument_id = result.unwrap();
        assert_eq!(instrument_id.symbol.as_str(), "BTCUSDT");
        assert_eq!(instrument_id.venue.as_str(), "BITGET");
    }

    #[test]
    fn test_instrument_id_to_symbol() {
        let instrument_id = InstrumentId::from("BTCUSDT.BITGET");
        let symbol = instrument_id_to_symbol(&instrument_id);
        assert_eq!(symbol.as_str(), "BTCUSDT");
    }
}

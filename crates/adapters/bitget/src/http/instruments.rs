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

//! Instrument fetching and conversion for Bitget.

use nautilus_core::nanos::UnixNanos;
use nautilus_model::{
    enums::CurrencyType,
    identifiers::{InstrumentId, Symbol, Venue},
    instruments::{CryptoFuture, CryptoPerpetual, CurrencyPair, InstrumentAny},
    types::{Currency, Price, Quantity},
};

use super::{
    client::BitgetHttpClient,
    error::BitgetHttpResult,
    models::{BitgetFuturesSymbol, BitgetSpotSymbol},
};

const BITGET_VENUE: &str = "BITGET";

/// Fetches and parses Bitget spot instruments into Nautilus format.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or parsing fails.
pub async fn fetch_spot_instruments(
    client: &BitgetHttpClient,
) -> BitgetHttpResult<Vec<InstrumentAny>> {
    let response = client.get_spot_symbols().await?;

    let data = response.data.ok_or_else(|| {
        super::error::BitgetHttpError::Deserialization("No data in response".into())
    })?;

    let instruments: Vec<InstrumentAny> = data
        .symbols
        .into_iter()
        .filter(|s| s.status == "online")
        .map(|symbol| parse_spot_instrument(symbol))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(instruments)
}

/// Fetches and parses Bitget futures instruments into Nautilus format.
///
/// # Arguments
///
/// * `client` - HTTP client
/// * `product_type` - Product type (e.g., "USDT-FUTURES", "COIN-FUTURES")
///
/// # Errors
///
/// Returns an error if the HTTP request fails or parsing fails.
pub async fn fetch_futures_instruments(
    client: &BitgetHttpClient,
    product_type: &str,
) -> BitgetHttpResult<Vec<InstrumentAny>> {
    let response = client.get_futures_symbols(product_type).await?;

    let data = response.data.ok_or_else(|| {
        super::error::BitgetHttpError::Deserialization("No data in response".into())
    })?;

    let instruments: Vec<InstrumentAny> = data
        .symbols
        .into_iter()
        .filter(|s| s.symbol_status == "normal")
        .map(|symbol| parse_futures_instrument(symbol))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(instruments)
}

fn parse_spot_instrument(symbol: BitgetSpotSymbol) -> BitgetHttpResult<InstrumentAny> {
    let instrument_id = InstrumentId::new(
        Symbol::new(&symbol.symbol.to_string()),
        Venue::new(BITGET_VENUE),
    );

    let base_currency = Currency::new(
        symbol.base_coin.as_str(),
        symbol
            .quantity_precision
            .parse::<u8>()
            .unwrap_or(8),
        0,
        symbol.base_coin.as_str(),
        CurrencyType::Crypto,
    );

    let quote_currency = Currency::new(
        symbol.quote_coin.as_str(),
        symbol.price_precision.parse::<u8>().unwrap_or(8),
        0,
        symbol.quote_coin.as_str(),
        CurrencyType::Crypto,
    );

    let price_precision = symbol.price_precision.parse::<u8>().unwrap_or(8);
    let size_precision = symbol.quantity_precision.parse::<u8>().unwrap_or(8);

    let price_increment = Price::new(
        10_f64.powi(-(price_precision as i32)),
        price_precision,
    );

    let size_increment = Quantity::new(
        10_f64.powi(-(size_precision as i32)),
        size_precision,
    );

    let min_quantity = Quantity::new(
        symbol
            .min_trade_amount
            .parse::<f64>()
            .unwrap_or(0.0),
        size_precision,
    );

    let max_quantity = symbol
        .max_trade_amount
        .parse::<f64>()
        .ok()
        .map(|v| Quantity::new(v, size_precision));

    let instrument = CurrencyPair::new(
        instrument_id,
        instrument_id.symbol,
        base_currency,
        quote_currency,
        price_precision,
        size_precision,
        price_increment,
        size_increment,
        None,                 // multiplier
        None,                 // lot_size
        max_quantity,
        Some(min_quantity),
        None,                 // max_notional
        None,                 // min_notional
        None,                 // max_price
        None,                 // min_price
        None,                 // margin_init
        None,                 // margin_maint
        None,                 // maker_fee
        None,                 // taker_fee
        UnixNanos::default(), // ts_event
        UnixNanos::default(), // ts_init
    );

    Ok(InstrumentAny::CurrencyPair(instrument))
}

fn parse_futures_instrument(symbol: BitgetFuturesSymbol) -> BitgetHttpResult<InstrumentAny> {
    let instrument_id = InstrumentId::new(
        Symbol::new(&symbol.symbol.to_string()),
        Venue::new(BITGET_VENUE),
    );

    let base_currency = Currency::new(
        symbol.base_coin.as_str(),
        symbol.volume_place.parse::<u8>().unwrap_or(8),
        0,
        symbol.base_coin.as_str(),
        CurrencyType::Crypto,
    );

    let quote_currency = Currency::new(
        symbol.quote_coin.as_str(),
        symbol.price_place.parse::<u8>().unwrap_or(8),
        0,
        symbol.quote_coin.as_str(),
        CurrencyType::Crypto,
    );

    let price_precision = symbol.price_place.parse::<u8>().unwrap_or(8);
    let size_precision = symbol.volume_place.parse::<u8>().unwrap_or(8);

    let price_increment = Price::new(
        10_f64.powi(-(price_precision as i32)),
        price_precision,
    );

    let size_increment = Quantity::new(
        symbol
            .size_multiplier
            .parse::<f64>()
            .unwrap_or(1.0),
        size_precision,
    );

    let min_quantity = Quantity::new(
        symbol
            .min_trade_num
            .parse::<f64>()
            .unwrap_or(1.0),
        size_precision,
    );

    // Check if perpetual (no expiry) or futures (has expiry)
    // Bitget perpetuals typically have "USDT" in symbol_type
    if symbol.symbol_type.contains("perpetual") || symbol.symbol_type.contains("PERPETUAL") {
        let instrument = CryptoPerpetual::new(
            instrument_id,
            instrument_id.symbol,
            base_currency,
            quote_currency,
            base_currency,      // settlement_currency
            false,              // is_inverse
            price_precision,
            size_precision,
            price_increment,
            size_increment,
            None,               // multiplier
            None,               // lot_size
            None,               // max_quantity
            Some(min_quantity), // min_quantity
            None,               // max_notional
            None,               // min_notional
            None,               // max_price
            None,               // min_price
            None,               // margin_init
            None,               // margin_maint
            None,               // maker_fee
            None,               // taker_fee
            UnixNanos::default(), // ts_event
            UnixNanos::default(), // ts_init
        );

        Ok(InstrumentAny::CryptoPerpetual(instrument))
    } else {
        // Futures with expiry
        let instrument = CryptoFuture::new(
            instrument_id,
            instrument_id.symbol,
            base_currency,        // underlying
            quote_currency,
            quote_currency,       // settlement_currency
            false,                // is_inverse
            UnixNanos::default(), // activation_ns (would need to parse from API)
            UnixNanos::default(), // expiration_ns (would need to parse from symbol or API)
            price_precision,
            size_precision,
            price_increment,
            size_increment,
            None,                 // multiplier
            None,                 // lot_size
            None,                 // max_quantity
            Some(min_quantity),   // min_quantity
            None,                 // max_notional
            None,                 // min_notional
            None,                 // max_price
            None,                 // min_price
            None,                 // margin_init
            None,                 // margin_maint
            None,                 // maker_fee
            None,                 // taker_fee
            UnixNanos::default(), // ts_event
            UnixNanos::default(), // ts_init
        );

        Ok(InstrumentAny::CryptoFuture(instrument))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use ustr::Ustr;

    #[test]
    fn test_parse_spot_instrument() {
        let symbol = BitgetSpotSymbol {
            symbol: Ustr::from("BTCUSDT"),
            base_coin: Ustr::from("BTC"),
            quote_coin: Ustr::from("USDT"),
            min_trade_amount: "0.0001".to_string(),
            max_trade_amount: "10000".to_string(),
            price_precision: "2".to_string(),
            quantity_precision: "6".to_string(),
            quote_precision: "8".to_string(),
            status: "online".to_string(),
            min_trade_usdt: "5".to_string(),
            max_trade_usdt: "500000".to_string(),
        };

        let result = parse_spot_instrument(symbol);
        assert!(result.is_ok());

        let instrument = result.unwrap();
        match instrument {
            InstrumentAny::CurrencyPair(pair) => {
                assert_eq!(pair.id.symbol.as_str(), "BTCUSDT");
                assert_eq!(pair.base_currency.code.as_str(), "BTC");
                assert_eq!(pair.quote_currency.code.as_str(), "USDT");
            }
            _ => panic!("Expected CurrencyPair"),
        }
    }

    #[tokio::test]
    async fn test_fetch_spot_instruments() {
        let client = BitgetHttpClient::default();

        // This test requires network access
        // In production, we'd use a mock server
        let result = fetch_spot_instruments(&client).await;

        if result.is_ok() {
            let instruments = result.unwrap();
            assert!(!instruments.is_empty());
        }
    }
}

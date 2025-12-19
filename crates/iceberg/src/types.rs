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

//! Type definitions for L3 orderbook events and Iceberg table schemas.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents an L3 orderbook event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderbookL3Event {
    /// Event timestamp in microseconds since epoch.
    pub timestamp: i64,
    /// Trading instrument identifier (e.g., "BTC-PERP").
    pub instrument_id: String,
    /// Exchange name (e.g., "BINANCE").
    pub exchange: String,
    /// Unique order identifier.
    pub order_id: String,
    /// Order action (ADD, UPDATE, DELETE).
    pub action: OrderAction,
    /// Order side (BUY, SELL).
    pub side: OrderSide,
    /// Order price (stored as decimal string for precision).
    pub price: String,
    /// Order quantity (stored as decimal string for precision).
    pub quantity: String,
    /// Monotonically increasing sequence number.
    pub sequence_number: u64,
}

/// Represents a trade event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeEvent {
    /// Trade timestamp in microseconds since epoch.
    pub timestamp: i64,
    /// Trading instrument identifier.
    pub instrument_id: String,
    /// Exchange name.
    pub exchange: String,
    /// Unique trade identifier.
    pub trade_id: String,
    /// Trade side (BUY, SELL).
    pub side: OrderSide,
    /// Trade price (stored as decimal string for precision).
    pub price: String,
    /// Trade quantity (stored as decimal string for precision).
    pub quantity: String,
    /// Side of the aggressor (BUY, SELL, or NONE).
    pub aggressor_side: String,
}

/// Represents a liquidation event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidationEvent {
    /// Liquidation timestamp in microseconds since epoch.
    pub timestamp: i64,
    /// Trading instrument identifier.
    pub instrument_id: String,
    /// Exchange name.
    pub exchange: String,
    /// Unique liquidation identifier.
    pub liquidation_id: String,
    /// Liquidation side (BUY, SELL).
    pub side: OrderSide,
    /// Liquidation price (stored as decimal string for precision).
    pub price: String,
    /// Liquidation quantity (stored as decimal string for precision).
    pub quantity: String,
}

/// Represents the action performed on an L3 order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderAction {
    /// A new order was added to the book.
    Add,
    /// An existing order was updated (price or quantity changed).
    Update,
    /// An order was deleted from the book.
    Delete,
}

impl OrderAction {
    /// Returns the string representation of the action.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "ADD",
            Self::Update => "UPDATE",
            Self::Delete => "DELETE",
        }
    }

    /// Converts from a string representation.
    ///
    /// Returns `None` if the string is not a valid action.
    ///
    /// # Note
    ///
    /// This is a convenience method. The type also implements `FromStr` trait.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "ADD" => Some(Self::Add),
            "UPDATE" => Some(Self::Update),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}

impl fmt::Display for OrderAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents the side of an order (Buy or Sell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    /// Buy side order.
    Buy,
    /// Sell side order.
    Sell,
}

impl OrderSide {
    /// Returns the string representation of the side.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }

    /// Converts from a string representation.
    ///
    /// Returns `None` if the string is not a valid side.
    ///
    /// # Note
    ///
    /// This is a convenience method. The type also implements `FromStr` trait.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            _ => None,
        }
    }

    /// Returns the opposite side.
    #[must_use]
    pub fn opposite(&self) -> Self {
        match self {
            Self::Buy => Self::Sell,
            Self::Sell => Self::Buy,
        }
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_action_as_str() {
        assert_eq!(OrderAction::Add.as_str(), "ADD");
        assert_eq!(OrderAction::Update.as_str(), "UPDATE");
        assert_eq!(OrderAction::Delete.as_str(), "DELETE");
    }

    #[test]
    fn test_order_action_from_str() {
        assert_eq!(OrderAction::from_str("ADD"), Some(OrderAction::Add));
        assert_eq!(OrderAction::from_str("add"), Some(OrderAction::Add));
        assert_eq!(OrderAction::from_str("UPDATE"), Some(OrderAction::Update));
        assert_eq!(OrderAction::from_str("update"), Some(OrderAction::Update));
        assert_eq!(OrderAction::from_str("DELETE"), Some(OrderAction::Delete));
        assert_eq!(OrderAction::from_str("delete"), Some(OrderAction::Delete));
        assert_eq!(OrderAction::from_str("INVALID"), None);
    }

    #[test]
    fn test_order_action_display() {
        assert_eq!(format!("{}", OrderAction::Add), "ADD");
        assert_eq!(format!("{}", OrderAction::Update), "UPDATE");
        assert_eq!(format!("{}", OrderAction::Delete), "DELETE");
    }

    #[test]
    fn test_order_action_serde() {
        let action = OrderAction::Add;
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(json, "\"ADD\"");
        let deserialized: OrderAction = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, action);
    }

    #[test]
    fn test_order_side_as_str() {
        assert_eq!(OrderSide::Buy.as_str(), "BUY");
        assert_eq!(OrderSide::Sell.as_str(), "SELL");
    }

    #[test]
    fn test_order_side_from_str() {
        assert_eq!(OrderSide::from_str("BUY"), Some(OrderSide::Buy));
        assert_eq!(OrderSide::from_str("buy"), Some(OrderSide::Buy));
        assert_eq!(OrderSide::from_str("SELL"), Some(OrderSide::Sell));
        assert_eq!(OrderSide::from_str("sell"), Some(OrderSide::Sell));
        assert_eq!(OrderSide::from_str("INVALID"), None);
    }

    #[test]
    fn test_order_side_opposite() {
        assert_eq!(OrderSide::Buy.opposite(), OrderSide::Sell);
        assert_eq!(OrderSide::Sell.opposite(), OrderSide::Buy);
    }

    #[test]
    fn test_order_side_display() {
        assert_eq!(format!("{}", OrderSide::Buy), "BUY");
        assert_eq!(format!("{}", OrderSide::Sell), "SELL");
    }

    #[test]
    fn test_order_side_serde() {
        let side = OrderSide::Buy;
        let json = serde_json::to_string(&side).unwrap();
        assert_eq!(json, "\"BUY\"");
        let deserialized: OrderSide = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, side);
    }
}

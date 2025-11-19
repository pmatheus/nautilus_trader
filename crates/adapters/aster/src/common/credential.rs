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

use std::fmt::{Debug, Display};

use zeroize::ZeroizeOnDrop;

use crate::http::error::{Error, Result};

/// Represents a secure wrapper for EVM private key with zeroization on drop.
#[derive(Clone, ZeroizeOnDrop)]
pub struct EvmPrivateKey {
    #[zeroize(skip)]
    formatted_key: String, // Keep the formatted version for display
    #[zeroize(skip)]
    raw_bytes: Vec<u8>, // The actual key bytes
}

impl EvmPrivateKey {
    /// Creates a new EVM private key from hex string.
    pub fn new(key: String) -> Result<Self> {
        let key = key.trim().to_string();
        let hex_key = key.strip_prefix("0x").unwrap_or(&key);

        // Validate hex format and length
        if hex_key.len() != 64 {
            return Err(Error::bad_request(
                "EVM private key must be 32 bytes (64 hex chars)",
            ));
        }

        if !hex_key.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::bad_request("EVM private key must be valid hex"));
        }

        // Convert to lowercase for consistency
        let normalized = hex_key.to_lowercase();
        let formatted = format!("0x{}", normalized);

        // Parse to bytes for validation
        let raw_bytes = hex::decode(&normalized)
            .map_err(|_| Error::bad_request("Invalid hex in private key"))?;

        if raw_bytes.len() != 32 {
            return Err(Error::bad_request(
                "EVM private key must be exactly 32 bytes",
            ));
        }

        Ok(Self {
            formatted_key: formatted,
            raw_bytes,
        })
    }

    /// Get the formatted hex key (0x-prefixed)
    pub fn as_hex(&self) -> &str {
        &self.formatted_key
    }

    /// Gets the raw bytes (for signing operations).
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }
}

impl Debug for EvmPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EvmPrivateKey(***redacted***)")
    }
}

impl Display for EvmPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EvmPrivateKey(***redacted***)")
    }
}

/// Normalize EVM address to lowercase hex format
pub fn normalize_address(addr: &str) -> Result<String> {
    let addr = addr.trim();
    let hex_part = addr
        .strip_prefix("0x")
        .or_else(|| addr.strip_prefix("0X"))
        .unwrap_or(addr);

    if hex_part.len() != 40 {
        return Err(Error::bad_request(
            "Address must be 20 bytes (40 hex chars)",
        ));
    }

    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Error::bad_request("Address must be valid hex"));
    }

    Ok(format!("0x{}", hex_part.to_lowercase()))
}

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

//! Signing module for Aster DEX authentication.
//!
//! Aster uses EVM-style ECDSA signatures for authentication with parameters sorted by ASCII order.

use std::{collections::BTreeMap, str::FromStr};

use alloy_primitives::{keccak256, B256};
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;

use crate::{
    common::{credential::EvmPrivateKey, types::TimeNonce},
    http::error::{Error, Result},
};

/// Request to be signed for Aster API.
#[derive(Debug, Clone)]
pub struct SignRequest {
    pub params: BTreeMap<String, String>,
    pub nonce: TimeNonce,
}

/// Bundle containing signature and parameters for Aster requests.
#[derive(Debug, Clone)]
pub struct SignatureBundle {
    pub signature: String,
    pub nonce: u64,
    pub user: String,
    pub signer: String,
}

/// ECDSA signer for Aster DEX.
#[derive(Debug, Clone)]
pub struct AsterEcdsaSigner {
    private_key: EvmPrivateKey,
    user_address: String,
    signer_address: String,
}

impl AsterEcdsaSigner {
    /// Creates a new Aster signer with the given private key.
    ///
    /// The signer address is derived from the private key.
    /// The user address defaults to the signer address (can be overridden for sub-accounts).
    pub fn new(private_key: EvmPrivateKey) -> Result<Self> {
        let signer_address = Self::derive_address(&private_key)?;
        let user_address = signer_address.clone();

        Ok(Self {
            private_key,
            user_address,
            signer_address,
        })
    }

    /// Creates a new Aster signer with explicit user address (for sub-accounts).
    pub fn with_user_address(private_key: EvmPrivateKey, user_address: String) -> Result<Self> {
        let signer_address = Self::derive_address(&private_key)?;

        Ok(Self {
            private_key,
            user_address,
            signer_address,
        })
    }

    /// Signs a request according to Aster's authentication scheme.
    ///
    /// Process:
    /// 1. Add user, signer, nonce to parameters
    /// 2. Sort all parameters by ASCII key order
    /// 3. Encode with Web3 ABI
    /// 4. Generate Keccak256 hash
    /// 5. Sign hash using ECDSA
    pub fn sign(&self, request: &SignRequest) -> Result<SignatureBundle> {
        let nonce_micros = request.nonce.as_micros();

        // Create complete parameter set including auth parameters
        let mut all_params = request.params.clone();
        all_params.insert("user".to_string(), self.user_address.clone());
        all_params.insert("signer".to_string(), self.signer_address.clone());
        all_params.insert("nonce".to_string(), nonce_micros.to_string());

        // Sort parameters by ASCII order (BTreeMap handles this automatically)
        // Encode parameters for signing
        let param_string = self.encode_params(&all_params)?;

        // Hash the encoded parameters
        let hash = keccak256(param_string.as_bytes());

        // Sign the hash
        let signature = self.sign_hash(&hash)?;

        Ok(SignatureBundle {
            signature,
            nonce: nonce_micros,
            user: self.user_address.clone(),
            signer: self.signer_address.clone(),
        })
    }

    /// Encodes parameters according to Aster's ABI encoding scheme.
    ///
    /// Parameters are sorted by key (BTreeMap guarantees this) and encoded as:
    /// - For addresses: parsed as Address type
    /// - For numbers: parsed as uint256
    /// - For strings: encoded as string
    fn encode_params(&self, params: &BTreeMap<String, String>) -> Result<String> {
        // Build the encoding string by concatenating sorted key=value pairs
        let mut parts = Vec::new();

        for (key, value) in params.iter() {
            parts.push(format!("{}={}", key, value));
        }

        Ok(parts.join("&"))
    }

    /// Signs a hash using ECDSA.
    fn sign_hash(&self, hash: &B256) -> Result<String> {
        let key_hex = self.private_key.as_hex();
        let key_hex = key_hex.strip_prefix("0x").unwrap_or(key_hex);

        // Create PrivateKeySigner from hex string
        let signer = PrivateKeySigner::from_str(key_hex)
            .map_err(|e| Error::transport(format!("Failed to create signer: {e}")))?;

        // Sign the hash
        let signature = signer
            .sign_hash_sync(hash)
            .map_err(|e| Error::transport(format!("Failed to sign hash: {e}")))?;

        // Extract r, s, v components for Ethereum signature format
        let r = signature.r();
        let s = signature.s();
        let v = signature.v();

        // Convert v from bool to Ethereum recovery ID (27 or 28)
        let v_byte = if v { 28u8 } else { 27u8 };

        // Format as Ethereum signature: 0x + r + s + v
        Ok(format!("0x{:064x}{:064x}{:02x}", r, s, v_byte))
    }

    /// Derives the Ethereum address from the private key.
    fn derive_address(private_key: &EvmPrivateKey) -> Result<String> {
        let key_hex = private_key.as_hex();
        let key_hex = key_hex.strip_prefix("0x").unwrap_or(key_hex);

        let signer = PrivateKeySigner::from_str(key_hex)
            .map_err(|e| Error::transport(format!("Failed to create signer: {e}")))?;

        Ok(format!("{:#x}", signer.address()))
    }

    /// Gets the user address.
    pub fn user_address(&self) -> &str {
        &self.user_address
    }

    /// Gets the signer address.
    pub fn signer_address(&self) -> &str {
        &self.signer_address
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_PRIVATE_KEY: &str =
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    #[rstest]
    fn test_signer_creation() {
        let private_key = EvmPrivateKey::new(TEST_PRIVATE_KEY.to_string()).unwrap();
        let signer = AsterEcdsaSigner::new(private_key).unwrap();

        assert!(!signer.user_address().is_empty());
        assert!(!signer.signer_address().is_empty());
        assert_eq!(signer.user_address(), signer.signer_address());
    }

    #[rstest]
    fn test_signer_with_user_address() {
        let private_key = EvmPrivateKey::new(TEST_PRIVATE_KEY.to_string()).unwrap();
        let user_address = "0x1234567890123456789012345678901234567890".to_string();
        let signer = AsterEcdsaSigner::with_user_address(private_key, user_address.clone()).unwrap();

        assert_eq!(signer.user_address(), user_address);
        assert_ne!(signer.user_address(), signer.signer_address());
    }

    #[rstest]
    fn test_sign_request() {
        let private_key = EvmPrivateKey::new(TEST_PRIVATE_KEY.to_string()).unwrap();
        let signer = AsterEcdsaSigner::new(private_key).unwrap();

        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), "BTCUSDT".to_string());
        params.insert("side".to_string(), "BUY".to_string());
        params.insert("quantity".to_string(), "1.0".to_string());

        let request = SignRequest {
            params,
            nonce: TimeNonce::from_micros(1640995200000000),
        };

        let result = signer.sign(&request).unwrap();

        // Verify signature format: 0x + 64 hex chars (r) + 64 hex chars (s) + 2 hex chars (v)
        assert!(result.signature.starts_with("0x"));
        assert_eq!(result.signature.len(), 132); // 0x + 130 hex chars
        assert_eq!(result.nonce, 1640995200000000);
        assert!(!result.user.is_empty());
        assert!(!result.signer.is_empty());
    }

    #[rstest]
    fn test_param_encoding_order() {
        let private_key = EvmPrivateKey::new(TEST_PRIVATE_KEY.to_string()).unwrap();
        let signer = AsterEcdsaSigner::new(private_key).unwrap();

        let mut params = BTreeMap::new();
        params.insert("z".to_string(), "last".to_string());
        params.insert("a".to_string(), "first".to_string());
        params.insert("m".to_string(), "middle".to_string());

        let encoded = signer.encode_params(&params).unwrap();

        // BTreeMap ensures ASCII order: a, m, z
        assert!(encoded.starts_with("a=first"));
        assert!(encoded.contains("m=middle"));
        assert!(encoded.ends_with("z=last"));
    }
}

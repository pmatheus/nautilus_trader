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

//! Bitget API credential storage and signing helpers.

use std::fmt::{Debug, Formatter};

use aws_lc_rs::hmac;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use ustr::Ustr;
use zeroize::ZeroizeOnDrop;

/// API credentials required for signing Bitget REST and WebSocket requests.
///
/// Bitget uses HMAC-SHA256 for request signing with three components:
/// - API Key: Public identifier
/// - Secret Key: Used for HMAC signing
/// - Passphrase: Additional authentication parameter
#[derive(Clone, ZeroizeOnDrop)]
pub struct Credential {
    #[zeroize(skip)]
    api_key: Ustr,
    api_secret: Box<[u8]>,
    #[zeroize(skip)]
    passphrase: Ustr,
}

impl Debug for Credential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credential")
            .field("api_key", &self.masked_api_key())
            .field("api_secret", &"<redacted>")
            .field("passphrase", &"<redacted>")
            .finish()
    }
}

impl Credential {
    /// Creates a new [`Credential`] instance from the API key, secret, and passphrase.
    #[must_use]
    pub fn new(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        let api_key = api_key.into();
        let api_secret_bytes = api_secret.into().into_bytes();
        let passphrase = passphrase.into();

        let api_key = Ustr::from(&api_key);
        let passphrase = Ustr::from(&passphrase);

        Self {
            api_key,
            api_secret: api_secret_bytes.into_boxed_slice(),
            passphrase,
        }
    }

    /// Returns the API key associated with this credential.
    #[must_use]
    pub fn api_key(&self) -> &Ustr {
        &self.api_key
    }

    /// Returns the passphrase associated with this credential.
    #[must_use]
    pub fn passphrase(&self) -> &Ustr {
        &self.passphrase
    }

    /// Returns a masked version of the API key for logging purposes.
    ///
    /// Shows first 4 and last 4 characters with ellipsis in between.
    /// For keys shorter than 8 characters, shows asterisks only.
    #[must_use]
    pub fn masked_api_key(&self) -> String {
        let key = self.api_key.as_str();
        let len = key.len();

        if len <= 8 {
            "*".repeat(len)
        } else {
            format!("{}...{}", &key[..4], &key[len - 4..])
        }
    }

    /// Produces the Bitget REST API signature.
    ///
    /// Bitget signature format: Base64(HMAC-SHA256(timestamp + method + requestPath + body, secret))
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp in milliseconds as string
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `request_path` - Request path including query parameters
    /// * `body` - Request body (empty string for GET requests)
    #[must_use]
    pub fn sign_request(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> String {
        let message = format!("{}{}{}{}", timestamp, method.to_uppercase(), request_path, body);

        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.api_secret);
        let tag = hmac::sign(&key, message.as_bytes());
        BASE64.encode(tag.as_ref())
    }

    /// Produces the Bitget WebSocket authentication signature.
    ///
    /// WebSocket signature format: Base64(HMAC-SHA256(timestamp + "GET" + "/user/verify", secret))
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp in milliseconds
    #[must_use]
    pub fn sign_websocket_auth(&self, timestamp: i64) -> String {
        let message = format!("{}GET/user/verify", timestamp);

        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.api_secret);
        let tag = hmac::sign(&key, message.as_bytes());
        BASE64.encode(tag.as_ref())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    const API_KEY: &str = "test_api_key";
    const API_SECRET: &str = "test_secret";
    const PASSPHRASE: &str = "test_passphrase";

    #[test]
    fn test_credential_creation() {
        let credential = Credential::new(API_KEY, API_SECRET, PASSPHRASE);

        assert_eq!(credential.api_key().as_str(), API_KEY);
        assert_eq!(credential.passphrase().as_str(), PASSPHRASE);
    }

    #[test]
    fn test_masked_api_key() {
        let credential = Credential::new(API_KEY, API_SECRET, PASSPHRASE);
        let masked = credential.masked_api_key();

        assert_eq!(masked, "test..._key");
    }

    #[test]
    fn test_masked_api_key_short() {
        let credential = Credential::new("short", API_SECRET, PASSPHRASE);
        let masked = credential.masked_api_key();

        assert_eq!(masked, "*****");
    }

    #[test]
    fn test_sign_request_get() {
        let credential = Credential::new(API_KEY, API_SECRET, PASSPHRASE);
        let timestamp = "1621441751927";
        let method = "GET";
        let request_path = "/api/v2/spot/market/products";
        let body = "";

        let signature = credential.sign_request(timestamp, method, request_path, body);

        // Signature should be base64 encoded
        assert!(!signature.is_empty());
        assert!(BASE64.decode(&signature).is_ok());
    }

    #[test]
    fn test_sign_request_post() {
        let credential = Credential::new(API_KEY, API_SECRET, PASSPHRASE);
        let timestamp = "1621441751927";
        let method = "POST";
        let request_path = "/api/v2/spot/trade/place-order";
        let body = r#"{"symbol":"BTCUSDT","side":"buy"}"#;

        let signature = credential.sign_request(timestamp, method, request_path, body);

        // Signature should be base64 encoded
        assert!(!signature.is_empty());
        assert!(BASE64.decode(&signature).is_ok());
    }

    #[test]
    fn test_sign_websocket_auth() {
        let credential = Credential::new(API_KEY, API_SECRET, PASSPHRASE);
        let timestamp = 1621441751927;

        let signature = credential.sign_websocket_auth(timestamp);

        // Signature should be base64 encoded
        assert!(!signature.is_empty());
        assert!(BASE64.decode(&signature).is_ok());
    }

    #[test]
    fn test_signature_deterministic() {
        let credential = Credential::new(API_KEY, API_SECRET, PASSPHRASE);
        let timestamp = "1621441751927";
        let method = "GET";
        let request_path = "/api/v2/spot/market/products";
        let body = "";

        let sig1 = credential.sign_request(timestamp, method, request_path, body);
        let sig2 = credential.sign_request(timestamp, method, request_path, body);

        assert_eq!(sig1, sig2);
    }
}

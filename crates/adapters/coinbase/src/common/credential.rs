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

//! Coinbase API credential storage and request signing helpers.

use std::fmt::{Debug, Formatter};

use aws_lc_rs::hmac;
use base64::prelude::*;
use ustr::Ustr;
use zeroize::ZeroizeOnDrop;

/// Coinbase API credentials for signing requests.
///
/// Uses HMAC SHA256 for request signing as per Coinbase API specifications.
/// Secrets are automatically zeroized on drop for security.
#[derive(Clone, ZeroizeOnDrop)]
pub struct Credential {
    #[zeroize(skip)]
    pub api_key: Ustr,
    pub api_passphrase: String,
    api_secret: Box<[u8]>,
}

impl Debug for Credential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Credential))
            .field("api_key", &self.api_key)
            .field("api_passphrase", &self.api_passphrase)
            .field("api_secret", &"<redacted>")
            .finish()
    }
}

impl Credential {
    /// Creates a new [`Credential`] instance.
    #[must_use]
    pub fn new(api_key: String, api_secret: String, api_passphrase: String) -> Self {
        // Coinbase API secret is base64 encoded, we need to decode it for HMAC signing
        let decoded_secret = BASE64_STANDARD
            .decode(api_secret.as_bytes())
            .expect("Failed to decode Coinbase API secret");

        Self {
            api_key: api_key.into(),
            api_passphrase,
            api_secret: decoded_secret.into_boxed_slice(),
        }
    }

    /// Signs a request message according to the Coinbase authentication scheme.
    ///
    /// The signature is created by:
    /// 1. Creating the prehash string: timestamp + method + requestPath + body
    /// 2. Signing with HMAC SHA256 using the decoded API secret
    /// 3. Base64 encoding the result
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp in seconds as string
    /// * `method` - HTTP method (GET, POST, DELETE)
    /// * `request_path` - Request path including query parameters
    /// * `body` - Request body (empty string for GET requests)
    pub fn sign(&self, timestamp: &str, method: &str, request_path: &str, body: &str) -> String {
        self.sign_bytes(timestamp, method, request_path, Some(body.as_bytes()))
    }

    /// Signs a request message using raw body bytes to avoid any UTF-8 conversion
    /// or re-serialization differences between the signed content and the bytes sent.
    pub fn sign_bytes(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: Option<&[u8]>,
    ) -> String {
        let mut message = Vec::with_capacity(
            timestamp.len() + method.len() + request_path.len() + body.map_or(0, |b| b.len()),
        );
        message.extend_from_slice(timestamp.as_bytes());
        message.extend_from_slice(method.as_bytes());
        message.extend_from_slice(request_path.as_bytes());
        if let Some(b) = body {
            message.extend_from_slice(b);
        }

        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.api_secret[..]);
        let tag = hmac::sign(&key, &message);
        BASE64_STANDARD.encode(tag.as_ref())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Test credentials from Coinbase documentation
    const API_KEY: &str = "test-api-key";
    const API_SECRET: &str = "dGVzdC1hcGktc2VjcmV0"; // "test-api-secret" base64 encoded
    const API_PASSPHRASE: &str = "test-passphrase";

    #[rstest]
    fn test_simple_get() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let signature = credential.sign("1640000000", "GET", "/accounts", "");

        assert!(!signature.is_empty());
        assert!(BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[rstest]
    fn test_get_with_query_params() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let signature = credential.sign("1640000000", "GET", "/orders?product_id=BTC-USD", "");

        assert!(!signature.is_empty());
        assert!(BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[rstest]
    fn test_post_with_json_body() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let body = r#"{"product_id":"BTC-USD","side":"buy","type":"limit","price":"100.00","size":"0.01"}"#;
        let signature = credential.sign("1640000000", "POST", "/orders", body);

        assert!(!signature.is_empty());
        assert!(BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[rstest]
    fn test_delete_request() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let signature = credential.sign(
            "1640000000",
            "DELETE",
            "/orders/d50ec984-77a8-460a-b958-66f114b0de9b",
            "",
        );

        assert!(!signature.is_empty());
        assert!(BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[rstest]
    fn test_debug_redacts_secret() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );
        let dbg_out = format!("{:?}", credential);
        assert!(dbg_out.contains("api_secret: \"<redacted>\""));
        assert!(!dbg_out.contains("test-api-secret"));
    }

    #[rstest]
    fn test_sign_bytes_matches_sign() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let body = r#"{"product_id":"BTC-USD","side":"buy"}"#;
        let sig_string = credential.sign("1640000000", "POST", "/orders", body);
        let sig_bytes = credential.sign_bytes(
            "1640000000",
            "POST",
            "/orders",
            Some(body.as_bytes()),
        );

        assert_eq!(sig_string, sig_bytes);
    }
}

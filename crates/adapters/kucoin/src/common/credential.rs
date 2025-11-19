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

//! Kucoin API credential storage and request signing helpers.

use std::fmt::{Debug, Formatter};

use aws_lc_rs::hmac;
use base64::prelude::*;
use ustr::Ustr;
use zeroize::ZeroizeOnDrop;

/// Kucoin API credentials for signing requests.
///
/// Uses HMAC SHA256 for request signing as per Kucoin API specifications.
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
            .field("api_passphrase", &"<redacted>")
            .field("api_secret", &"<redacted>")
            .finish()
    }
}

impl Credential {
    /// Creates a new [`Credential`] instance.
    #[must_use]
    pub fn new(api_key: String, api_secret: String, api_passphrase: String) -> Self {
        Self {
            api_key: api_key.into(),
            api_passphrase,
            api_secret: api_secret.into_bytes().into_boxed_slice(),
        }
    }

    /// Signs a request message according to the Kucoin authentication scheme.
    ///
    /// Kucoin signature = base64(hmac-sha256(secret, timestamp + method + endpoint + body))
    ///
    /// # Parameters
    ///
    /// * `timestamp` - Unix timestamp in milliseconds
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `endpoint` - API endpoint path (e.g., "/api/v1/orders")
    /// * `body` - Request body (empty string for GET requests)
    pub fn sign(&self, timestamp: &str, method: &str, endpoint: &str, body: &str) -> String {
        self.sign_bytes(timestamp, method, endpoint, Some(body.as_bytes()))
    }

    /// Signs a request message using raw body bytes.
    ///
    /// # Parameters
    ///
    /// * `timestamp` - Unix timestamp in milliseconds
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `endpoint` - API endpoint path
    /// * `body` - Optional request body bytes
    pub fn sign_bytes(
        &self,
        timestamp: &str,
        method: &str,
        endpoint: &str,
        body: Option<&[u8]>,
    ) -> String {
        let mut message = Vec::with_capacity(
            timestamp.len() + method.len() + endpoint.len() + body.map_or(0, |b| b.len()),
        );
        message.extend_from_slice(timestamp.as_bytes());
        message.extend_from_slice(method.as_bytes());
        message.extend_from_slice(endpoint.as_bytes());
        if let Some(b) = body {
            message.extend_from_slice(b);
        }

        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.api_secret[..]);
        let tag = hmac::sign(&key, &message);
        BASE64_STANDARD.encode(tag.as_ref())
    }

    /// Signs the API passphrase according to the Kucoin authentication scheme.
    ///
    /// Kucoin requires the passphrase to be signed with the API secret.
    /// Signed passphrase = base64(hmac-sha256(secret, passphrase))
    #[must_use]
    pub fn sign_passphrase(&self) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &self.api_secret[..]);
        let tag = hmac::sign(&key, self.api_passphrase.as_bytes());
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

    const API_KEY: &str = "5c2db93503aa674c74a31734";
    const API_SECRET: &str = "f03a5284-5c39-4aaa-9b20-dea10bdcf8e3";
    const API_PASSPHRASE: &str = "QWIxMjM0NTY3OCkoKiZeJSQjQA==";

    #[rstest]
    fn test_sign_get_request() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let signature = credential.sign("1547015186532", "GET", "/api/v1/accounts", "");

        assert!(!signature.is_empty());
        assert!(BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[rstest]
    fn test_sign_post_request() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let body = r#"{"side":"buy","symbol":"BTC-USDT","type":"limit","price":"1000","size":"0.01"}"#;
        let signature = credential.sign("1547015186532", "POST", "/api/v1/orders", body);

        assert!(!signature.is_empty());
        assert!(BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[rstest]
    fn test_sign_passphrase() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let signed_passphrase = credential.sign_passphrase();

        assert!(!signed_passphrase.is_empty());
        assert!(BASE64_STANDARD.decode(&signed_passphrase).is_ok());
    }

    #[rstest]
    fn test_debug_redacts_secrets() {
        let credential = Credential::new(
            API_KEY.to_string(),
            API_SECRET.to_string(),
            API_PASSPHRASE.to_string(),
        );

        let dbg_out = format!("{:?}", credential);
        assert!(dbg_out.contains("api_key"));
        assert!(dbg_out.contains("api_passphrase: \"<redacted>\""));
        assert!(dbg_out.contains("api_secret: \"<redacted>\""));
        assert!(!dbg_out.contains(API_SECRET));
        assert!(!dbg_out.contains(API_PASSPHRASE));
    }
}

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

//! Configuration types for the Kucoin adapter.

use std::sync::Arc;

use nautilus_core::env::get_or_env_var;
use serde::{Deserialize, Serialize};

use crate::common::{credential::Credential, enums::KucoinProductType};

/// Configuration for the Kucoin HTTP and WebSocket clients.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KucoinClientConfig {
    /// API key for authentication (if None, sourced from `KUCOIN_API_KEY` environment variable)
    pub api_key: Option<String>,
    /// API secret for authentication (if None, sourced from `KUCOIN_API_SECRET` environment variable)
    pub api_secret: Option<String>,
    /// API passphrase for authentication (if None, sourced from `KUCOIN_PASSPHRASE` environment variable)
    pub api_passphrase: Option<String>,
    /// Product type (SPOT or FUTURES)
    pub product_type: KucoinProductType,
    /// Base URL for HTTP API (defaults to production)
    pub base_url_http: Option<String>,
    /// Base URL for WebSocket API (defaults to production)
    pub base_url_ws: Option<String>,
    /// HTTP timeout in seconds
    pub http_timeout_secs: Option<u64>,
    /// Maximum number of retries for failed requests
    pub max_retries: Option<u32>,
    /// Whether this is a sandbox environment
    pub is_sandbox: bool,
}

impl Default for KucoinClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            api_passphrase: None,
            product_type: KucoinProductType::Spot,
            base_url_http: None,
            base_url_ws: None,
            http_timeout_secs: Some(60),
            max_retries: Some(3),
            is_sandbox: false,
        }
    }
}

impl KucoinClientConfig {
    /// Creates a new [`KucoinClientConfig`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a credential from the configuration, sourcing from environment variables if needed.
    ///
    /// # Errors
    ///
    /// Returns an error if required credentials are missing.
    pub fn credential(&self) -> anyhow::Result<Option<Arc<Credential>>> {
        let api_key = get_or_env_var(&self.api_key, "KUCOIN_API_KEY");
        let api_secret = get_or_env_var(&self.api_secret, "KUCOIN_API_SECRET");
        let api_passphrase = get_or_env_var(&self.api_passphrase, "KUCOIN_PASSPHRASE");

        if api_key.is_none() && api_secret.is_none() && api_passphrase.is_none() {
            return Ok(None);
        }

        let api_key = api_key.ok_or_else(|| anyhow::anyhow!("Missing KUCOIN_API_KEY"))?;
        let api_secret = api_secret.ok_or_else(|| anyhow::anyhow!("Missing KUCOIN_API_SECRET"))?;
        let api_passphrase =
            api_passphrase.ok_or_else(|| anyhow::anyhow!("Missing KUCOIN_PASSPHRASE"))?;

        Ok(Some(Arc::new(Credential::new(
            api_key,
            api_secret,
            api_passphrase,
        ))))
    }
}

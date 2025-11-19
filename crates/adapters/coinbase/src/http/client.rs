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

//! HTTP client for Coinbase Exchange API.

use std::sync::Arc;

use chrono::Utc;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::trace;

use super::error::{CoinbaseHttpError, Result};
use crate::common::{
    consts::*, credential::Credential, models::CoinbaseProduct, CoinbaseAccount,
};

/// HTTP client for Coinbase Exchange REST API.
#[derive(Clone)]
pub struct CoinbaseHttpClient {
    base_url: String,
    client: reqwest::Client,
    credential: Option<Arc<Credential>>,
}

impl CoinbaseHttpClient {
    /// Creates a new HTTP client.
    pub fn new(base_url: String, credential: Option<Credential>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("nautilus-coinbase/0.1.0")
            .build()?;

        Ok(Self {
            base_url,
            client,
            credential: credential.map(Arc::new),
        })
    }

    /// Makes an authenticated request.
    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.request(method.clone(), &url);

        // Add authentication headers if credentials are available
        if let Some(credential) = &self.credential {
            let timestamp = Utc::now().timestamp().to_string();
            let body_str = body
                .as_ref()
                .map(|b| serde_json::to_string(b).unwrap_or_default())
                .unwrap_or_default();

            let signature = credential.sign(
                &timestamp,
                method.as_str(),
                path,
                &body_str,
            );

            request = request
                .header(COINBASE_HEADER_ACCESS_KEY, credential.api_key.as_str())
                .header(COINBASE_HEADER_ACCESS_SIGN, signature)
                .header(COINBASE_HEADER_ACCESS_TIMESTAMP, timestamp)
                .header(COINBASE_HEADER_ACCESS_PASSPHRASE, &credential.api_passphrase);
        }

        // Add body if present
        if let Some(body) = body {
            request = request.json(&body);
        }

        trace!("Sending {} request to {}", method, url);
        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(CoinbaseHttpError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }

        let data: T = response.json().await?;
        Ok(data)
    }

    /// Fetches all products.
    pub async fn get_products(&self) -> Result<Vec<CoinbaseProduct>> {
        self.send_request(Method::GET, "/products", None).await
    }

    /// Fetches a specific product.
    pub async fn get_product(&self, product_id: &str) -> Result<CoinbaseProduct> {
        let path = format!("/products/{}", product_id);
        self.send_request(Method::GET, &path, None).await
    }

    /// Fetches all accounts.
    pub async fn get_accounts(&self) -> Result<Vec<CoinbaseAccount>> {
        self.send_request(Method::GET, "/accounts", None).await
    }

    /// Fetches a specific account.
    pub async fn get_account(&self, account_id: &str) -> Result<CoinbaseAccount> {
        let path = format!("/accounts/{}", account_id);
        self.send_request(Method::GET, &path, None).await
    }

    /// Places a new order.
    pub async fn place_order(&self, order: Value) -> Result<Value> {
        self.send_request(Method::POST, "/orders", Some(order))
            .await
    }

    /// Cancels an order.
    pub async fn cancel_order(&self, order_id: &str) -> Result<Value> {
        let path = format!("/orders/{}", order_id);
        self.send_request(Method::DELETE, &path, None).await
    }

    /// Cancels all orders for a product.
    pub async fn cancel_all_orders(&self, product_id: Option<&str>) -> Result<Vec<String>> {
        let path = if let Some(product_id) = product_id {
            format!("/orders?product_id={}", product_id)
        } else {
            "/orders".to_string()
        };
        self.send_request(Method::DELETE, &path, None).await
    }

    /// Fetches an order by ID.
    pub async fn get_order(&self, order_id: &str) -> Result<Value> {
        let path = format!("/orders/{}", order_id);
        self.send_request(Method::GET, &path, None).await
    }

    /// Lists all orders (optionally filtered by product).
    pub async fn list_orders(&self, product_id: Option<&str>) -> Result<Vec<Value>> {
        let path = if let Some(product_id) = product_id {
            format!("/orders?product_id={}", product_id)
        } else {
            "/orders".to_string()
        };
        self.send_request(Method::GET, &path, None).await
    }

    /// Fetches fills (trades).
    pub async fn get_fills(&self, product_id: Option<&str>) -> Result<Vec<Value>> {
        let path = if let Some(product_id) = product_id {
            format!("/fills?product_id={}", product_id)
        } else {
            "/fills".to_string()
        };
        self.send_request(Method::GET, &path, None).await
    }
}

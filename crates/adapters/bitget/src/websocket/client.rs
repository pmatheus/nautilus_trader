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

//! Bitget WebSocket client providing public market data and private account streaming.
//!
//! Bitget API reference <https://www.bitget.com/api-doc/common/intro>.

use std::{
    fmt::Debug,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use chrono::Utc;
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::{mpsc, RwLock},
    time::{interval, sleep},
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    connect_async,
    tungstenite::Message,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use ustr::Ustr;

use super::{
    enums::{BitgetInstType, BitgetWsOperation},
    error::{BitgetWsError, BitgetWsResult},
    messages::{
        BitgetAuthArg, BitgetAuthRequest, BitgetPing, BitgetSubscription, BitgetSubscriptionArg,
        BitgetWsMessage,
    },
};
use crate::common::{
    credential::Credential,
    enums::BitgetEnvironment,
    urls::{bitget_ws_private_url, bitget_ws_public_url},
};

const DEFAULT_HEARTBEAT_SECS: u64 = 20;
const RECONNECT_BASE_DELAY_MS: u64 = 1000;
const RECONNECT_MAX_DELAY_MS: u64 = 30000;
const MAX_RECONNECT_ATTEMPTS: usize = 10;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Public/market data or private WebSocket client for Bitget.
pub struct BitgetWebSocketClient {
    url: String,
    environment: BitgetEnvironment,
    credential: Option<Credential>,
    requires_auth: bool,
    heartbeat_interval: u64,
    is_connected: Arc<AtomicBool>,
    is_authenticated: Arc<AtomicBool>,
    subscriptions: Arc<DashMap<String, BitgetSubscriptionArg>>,
    message_tx: Arc<RwLock<Option<mpsc::UnboundedSender<BitgetWsMessage>>>>,
    cancellation_token: CancellationToken,
    reconnect_attempts: Arc<RwLock<usize>>,
}

impl Debug for BitgetWebSocketClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitgetWebSocketClient")
            .field("url", &self.url)
            .field("environment", &self.environment)
            .field("requires_auth", &self.requires_auth)
            .field("is_connected", &self.is_connected.load(Ordering::Relaxed))
            .field("subscriptions", &self.subscriptions.len())
            .finish()
    }
}

impl BitgetWebSocketClient {
    /// Creates a new [`BitgetWebSocketClient`] instance.
    ///
    /// # Arguments
    ///
    /// * `environment` - Trading environment (mainnet or testnet)
    /// * `is_private` - Whether this is a private (authenticated) channel
    /// * `api_key` - Optional API key for private channels
    /// * `api_secret` - Optional API secret for private channels
    /// * `passphrase` - Optional passphrase for private channels
    #[must_use]
    pub fn new(
        environment: BitgetEnvironment,
        is_private: bool,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        let url = if is_private {
            bitget_ws_private_url(environment).to_string()
        } else {
            bitget_ws_public_url(environment).to_string()
        };

        let credential = match (api_key, api_secret, passphrase) {
            (Some(key), Some(secret), Some(pass)) => Some(Credential::new(key, secret, pass)),
            _ => None,
        };

        let requires_auth = is_private;

        Self {
            url,
            environment,
            credential,
            requires_auth,
            heartbeat_interval: DEFAULT_HEARTBEAT_SECS,
            is_connected: Arc::new(AtomicBool::new(false)),
            is_authenticated: Arc::new(AtomicBool::new(false)),
            subscriptions: Arc::new(DashMap::new()),
            message_tx: Arc::new(RwLock::new(None)),
            cancellation_token: CancellationToken::new(),
            reconnect_attempts: Arc::new(RwLock::new(0)),
        }
    }

    /// Connects to the WebSocket and starts message processing.
    ///
    /// Returns a receiver for incoming messages.
    pub async fn connect(&self) -> BitgetWsResult<mpsc::UnboundedReceiver<BitgetWsMessage>> {
        let (tx, rx) = mpsc::unbounded_channel();
        *self.message_tx.write().await = Some(tx.clone());

        self.start_connection_loop(tx).await?;

        Ok(rx)
    }

    async fn start_connection_loop(
        &self,
        tx: mpsc::UnboundedSender<BitgetWsMessage>,
    ) -> BitgetWsResult<()> {
        let url = self.url.clone();
        let requires_auth = self.requires_auth;
        let credential = self.credential.clone();
        let is_connected = Arc::clone(&self.is_connected);
        let is_authenticated = Arc::clone(&self.is_authenticated);
        let subscriptions = Arc::clone(&self.subscriptions);
        let heartbeat_interval = self.heartbeat_interval;
        let cancellation_token = self.cancellation_token.clone();
        let reconnect_attempts = Arc::clone(&self.reconnect_attempts);

        tokio::spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    info!("WebSocket connection loop cancelled");
                    break;
                }

                match Self::connect_and_run(
                    &url,
                    requires_auth,
                    &credential,
                    &tx,
                    &is_connected,
                    &is_authenticated,
                    &subscriptions,
                    heartbeat_interval,
                    &cancellation_token,
                )
                .await
                {
                    Ok(()) => {
                        info!("WebSocket connection closed gracefully");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        is_connected.store(false, Ordering::Relaxed);
                        is_authenticated.store(false, Ordering::Relaxed);

                        let mut attempts = reconnect_attempts.write().await;
                        *attempts += 1;

                        if *attempts >= MAX_RECONNECT_ATTEMPTS {
                            error!("Max reconnection attempts reached, stopping");
                            let _ = tx.send(BitgetWsMessage::Error(
                                super::messages::BitgetWebSocketError {
                                    event: "max_reconnect".to_string(),
                                    code: "RECONNECT_FAILED".to_string(),
                                    msg: format!("Failed after {} attempts", *attempts),
                                },
                            ));
                            break;
                        }

                        let delay_ms = RECONNECT_BASE_DELAY_MS
                            * 2_u64.pow((*attempts as u32).min(5));
                        let delay_ms = delay_ms.min(RECONNECT_MAX_DELAY_MS);

                        warn!(
                            "Reconnecting in {}ms (attempt {}/{})",
                            delay_ms, *attempts, MAX_RECONNECT_ATTEMPTS
                        );

                        sleep(Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        });

        Ok(())
    }

    async fn connect_and_run(
        url: &str,
        requires_auth: bool,
        credential: &Option<Credential>,
        tx: &mpsc::UnboundedSender<BitgetWsMessage>,
        is_connected: &Arc<AtomicBool>,
        is_authenticated: &Arc<AtomicBool>,
        subscriptions: &Arc<DashMap<String, BitgetSubscriptionArg>>,
        heartbeat_interval: u64,
        cancellation_token: &CancellationToken,
    ) -> BitgetWsResult<()> {
        info!("Connecting to {}", url);

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| BitgetWsError::Connection(e.to_string()))?;

        info!("WebSocket connected successfully");
        is_connected.store(true, Ordering::Relaxed);

        let (mut write, mut read) = ws_stream.split();

        // Authenticate if required
        if requires_auth {
            if let Some(cred) = credential {
                let timestamp = Utc::now().timestamp_millis();
                let signature = cred.sign_websocket_auth(timestamp);

                let auth_request = BitgetAuthRequest {
                    op: BitgetWsOperation::Login,
                    args: vec![BitgetAuthArg {
                        api_key: cred.api_key().to_string(),
                        passphrase: cred.passphrase().to_string(),
                        timestamp: timestamp.to_string(),
                        sign: signature,
                    }],
                };

                let auth_msg = serde_json::to_string(&auth_request)
                    .map_err(|e| BitgetWsError::Serialization(e.to_string()))?;

                write
                    .send(Message::Text(auth_msg.into()))
                    .await
                    .map_err(|e| BitgetWsError::Send(e.to_string()))?;

                debug!("Authentication request sent");
            } else {
                return Err(BitgetWsError::Authentication(
                    "No credentials provided for private channel".into(),
                ));
            }
        }

        // Spawn heartbeat task
        let _heartbeat_tx = tx.clone();
        let mut heartbeat_timer = interval(Duration::from_secs(heartbeat_interval));
        let heartbeat_token = cancellation_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = heartbeat_timer.tick() => {
                        let ping = BitgetPing {
                            op: BitgetWsOperation::Ping,
                        };
                        if let Ok(_ping_msg) = serde_json::to_string(&ping) {
                            // Note: In production, we'd send this through a channel to the write task
                            debug!("Heartbeat ping would be sent");
                        }
                    }
                    _ = heartbeat_token.cancelled() => {
                        debug!("Heartbeat task cancelled");
                        break;
                    }
                }
            }
        });

        // Process incoming messages
        while let Some(msg_result) = read.next().await {
            if cancellation_token.is_cancelled() {
                break;
            }

            match msg_result {
                Ok(Message::Text(text)) => {
                    debug!("Received: {}", text);

                    // Parse and route message
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                        // Check for authentication response
                        if let Some(event) = value.get("event").and_then(|v| v.as_str()) {
                            if event == "login" {
                                if let Some(code) = value.get("code").and_then(|v| v.as_str()) {
                                    if code == "0" {
                                        info!("Authentication successful");
                                        is_authenticated.store(true, Ordering::Relaxed);

                                        // Resubscribe to all channels
                                        Self::resubscribe_all(subscriptions, &mut write).await?;
                                    }
                                }
                            }
                        }

                        // Send raw message for now (handler will parse it later)
                        let _ = tx.send(BitgetWsMessage::Raw(value));
                    }
                }
                Ok(Message::Ping(_)) => {
                    debug!("Received ping");
                }
                Ok(Message::Pong(_)) => {
                    let _ = tx.send(BitgetWsMessage::Pong);
                }
                Ok(Message::Close(frame)) => {
                    info!("Received close frame: {:?}", frame);
                    return Err(BitgetWsError::Closed("Server closed connection".into()));
                }
                Ok(Message::Binary(_)) => {
                    warn!("Received unexpected binary message");
                }
                Ok(Message::Frame(_)) => {
                    // Internal frame, ignore
                }
                Err(e) => {
                    return Err(BitgetWsError::Receive(e.to_string()));
                }
            }
        }

        Ok(())
    }

    async fn resubscribe_all(
        subscriptions: &Arc<DashMap<String, BitgetSubscriptionArg>>,
        write: &mut futures_util::stream::SplitSink<WsStream, Message>,
    ) -> BitgetWsResult<()> {
        if subscriptions.is_empty() {
            return Ok(());
        }

        let args: Vec<BitgetSubscriptionArg> = subscriptions
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        let subscription = BitgetSubscription {
            op: BitgetWsOperation::Subscribe,
            args,
        };

        let msg = serde_json::to_string(&subscription)
            .map_err(|e| BitgetWsError::Serialization(e.to_string()))?;

        write
            .send(Message::Text(msg.into()))
            .await
            .map_err(|e| BitgetWsError::Send(e.to_string()))?;

        info!("Resubscribed to {} channels", subscriptions.len());
        Ok(())
    }

    /// Subscribes to a channel.
    ///
    /// # Arguments
    ///
    /// * `inst_type` - Instrument type (e.g., SPOT, USDT-FUTURES)
    /// * `channel` - Channel name (e.g., "trade", "books")
    /// * `inst_id` - Instrument ID (e.g., "BTCUSDT")
    pub async fn subscribe(
        &self,
        inst_type: BitgetInstType,
        channel: &str,
        inst_id: &str,
    ) -> BitgetWsResult<()> {
        let sub_arg = BitgetSubscriptionArg {
            inst_type,
            channel: channel.to_string(),
            inst_id: Ustr::from(inst_id),
        };

        let key = format!("{}:{}:{}", inst_type, channel, inst_id);
        self.subscriptions.insert(key, sub_arg.clone());

        // If connected, send subscription immediately
        if self.is_connected.load(Ordering::Relaxed) {
            // In a full implementation, we'd send this through a command channel
            // For now, we store it and it will be sent on next reconnect
            debug!("Subscription queued: {:?}", sub_arg);
        }

        Ok(())
    }

    /// Unsubscribes from a channel.
    pub async fn unsubscribe(
        &self,
        inst_type: BitgetInstType,
        channel: &str,
        inst_id: &str,
    ) -> BitgetWsResult<()> {
        let key = format!("{}:{}:{}", inst_type, channel, inst_id);
        self.subscriptions.remove(&key);

        Ok(())
    }

    /// Returns `true` if connected.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::Relaxed)
    }

    /// Returns `true` if authenticated (for private channels).
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated.load(Ordering::Relaxed)
    }

    /// Disconnects and shuts down the client.
    pub async fn disconnect(&self) {
        info!("Disconnecting WebSocket client");
        self.cancellation_token.cancel();
        self.is_connected.store(false, Ordering::Relaxed);
        self.is_authenticated.store(false, Ordering::Relaxed);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_public() {
        let client = BitgetWebSocketClient::new(
            BitgetEnvironment::Mainnet,
            false,
            None,
            None,
            None,
        );

        assert!(!client.requires_auth);
        assert!(client.credential.is_none());
        assert_eq!(client.url, "wss://ws.bitget.com/v2/ws/public");
    }

    #[test]
    fn test_client_creation_private() {
        let client = BitgetWebSocketClient::new(
            BitgetEnvironment::Mainnet,
            true,
            Some("key".to_string()),
            Some("secret".to_string()),
            Some("pass".to_string()),
        );

        assert!(client.requires_auth);
        assert!(client.credential.is_some());
        assert_eq!(client.url, "wss://ws.bitget.com/v2/ws/private");
    }

    #[tokio::test]
    async fn test_subscribe() {
        let client = BitgetWebSocketClient::new(
            BitgetEnvironment::Mainnet,
            false,
            None,
            None,
            None,
        );

        let result = client
            .subscribe(BitgetInstType::Spot, "trade", "BTCUSDT")
            .await;

        assert!(result.is_ok());
        assert_eq!(client.subscriptions.len(), 1);
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let client = BitgetWebSocketClient::new(
            BitgetEnvironment::Mainnet,
            false,
            None,
            None,
            None,
        );

        client
            .subscribe(BitgetInstType::Spot, "trade", "BTCUSDT")
            .await
            .unwrap();

        client
            .unsubscribe(BitgetInstType::Spot, "trade", "BTCUSDT")
            .await
            .unwrap();

        assert_eq!(client.subscriptions.len(), 0);
    }
}

# Kucoin Token-Based WebSocket Connection Flow

## Overview

This document explains the unique token-based WebSocket connection system used by Kucoin, which differs significantly from traditional WebSocket implementations.

## Traditional WebSocket Flow (Most Exchanges)

```
Client                                Exchange
  |                                      |
  |  WS Connect: wss://exchange.com     |
  |------------------------------------->|
  |                                      |
  |  <---- Welcome/Connected ------------|
  |                                      |
  |  Subscribe to channel               |
  |------------------------------------->|
  |                                      |
  |  <---- Data streaming ---------------|
```

## Kucoin's Token-Based Flow

```
Client                                Kucoin REST API          Kucoin WebSocket
  |                                      |                           |
  |  1. POST /api/v1/bullet-public      |                           |
  |     (or /api/v1/bullet-private)     |                           |
  |------------------------------------->|                           |
  |                                      |                           |
  |  2. Token Response:                 |                           |
  |     {                                |                           |
  |       token: "abc123...",            |                           |
  |       instance_servers: [{           |                           |
  |         endpoint: "wss://...",       |                           |
  |         ping_interval: 18000,        |                           |
  |         ping_timeout: 10000          |                           |
  |       }]                             |                           |
  |     }                                |                           |
  |<-------------------------------------|                           |
  |                                      |                           |
  |  3. WS Connect:                                                  |
  |     wss://ws-api-spot.kucoin.com/?token=abc123...               |
  |----------------------------------------------------------------->|
  |                                                                  |
  |  4. Welcome Message:                                             |
  |     {                                                            |
  |       id: "xyz",                                                 |
  |       type: "welcome"                                            |
  |     }                                                            |
  |<-----------------------------------------------------------------|
  |                                                                  |
  |  5. Subscribe to channel:                                        |
  |     {                                                            |
  |       id: "1",                                                   |
  |       type: "subscribe",                                         |
  |       topic: "/market/ticker:BTC-USDT"                           |
  |     }                                                            |
  |----------------------------------------------------------------->|
  |                                                                  |
  |  6. Subscription ACK                                             |
  |<-----------------------------------------------------------------|
  |                                                                  |
  |  7. Data streaming                                               |
  |<-----------------------------------------------------------------|
  |                                                                  |
  |  8. Ping every 18 seconds                                        |
  |     { id: "2", type: "ping" }                                    |
  |----------------------------------------------------------------->|
  |                                                                  |
  |  9. Pong response                                                |
  |     { id: "2", type: "pong" }                                    |
  |<-----------------------------------------------------------------|
```

## Key Differences

### 1. Two-Step Connection Process

**Traditional**:
- Direct WebSocket connection
- Optional authentication via message

**Kucoin**:
- Must request token via REST API first
- Use token to establish WebSocket connection
- Token is embedded in URL query parameter

### 2. Token Types

**Public Token** (`/api/v1/bullet-public`):
- No authentication required
- For public market data (ticker, orderbook, trades)
- Anyone can request

**Private Token** (`/api/v1/bullet-private`):
- Requires API key authentication
- For private data (account updates, orders, balances)
- Includes authenticated channels

### 3. Token Validity

- **Duration**: 24 hours
- **Refresh**: Must request new token before expiration
- **Recommendation**: Refresh every 23 hours or on connection error

### 4. Server Selection

Response includes multiple WebSocket servers:
```json
{
  "token": "abc123...",
  "instance_servers": [
    {
      "endpoint": "wss://ws-api-spot.kucoin.com",
      "encrypt": true,
      "protocol": "websocket",
      "ping_interval": 18000,
      "ping_timeout": 10000
    },
    {
      "endpoint": "wss://ws-api-spot-backup.kucoin.com",
      "encrypt": true,
      "protocol": "websocket",
      "ping_interval": 18000,
      "ping_timeout": 10000
    }
  ]
}
```

**Strategy**:
- Use first server primarily
- Failover to other servers if connection fails

### 5. Welcome Message

**Critical**: Connection is NOT ready until welcome message received!

```json
{
  "id": "hQvf8jkno",
  "type": "welcome"
}
```

**Implementation**:
- Wait for welcome before sending subscriptions
- Timeout if welcome not received within 10 seconds
- Queue subscriptions until welcome received

### 6. Message Format

All WebSocket messages have this structure:
```json
{
  "id": "unique-message-id",
  "type": "message|subscribe|ack|error|welcome|pong",
  "topic": "/market/ticker:BTC-USDT",
  "subject": "trade.ticker",
  "data": { ... }
}
```

## Implementation in Nautilus

### Step 1: Token Acquisition

```rust
// In KucoinHttpClient
pub async fn get_public_ws_token(&self) -> Result<KucoinWsTokenResponse, KucoinHttpError> {
    self.send_public(Method::POST, "/api/v1/bullet-public").await
}

pub async fn get_private_ws_token(&self) -> Result<KucoinWsTokenResponse, KucoinHttpError> {
    self.send_authenticated(Method::POST, "/api/v1/bullet-private", None).await
}
```

### Step 2: Connection URL Construction

```rust
// In KucoinWebSocketClient
pub async fn get_ws_connection_url(&self) -> Result<String, KucoinWsError> {
    // Get token based on credentials
    let token_response = if self.credential.is_some() {
        self.http_client.get_private_ws_token().await?
    } else {
        self.http_client.get_public_ws_token().await?
    };

    // Extract first server
    let server = token_response.instance_servers.first().ok_or(...)?;

    // Construct URL with token
    let url = format!("{}?token={}", server.endpoint, token_response.token);
    Ok(url)
}
```

### Step 3: Connection & Welcome (To Be Completed)

```rust
// Pseudocode for full implementation
async fn connect(&self) -> Result<(), KucoinWsError> {
    // Get connection URL with token
    let url = self.get_ws_connection_url().await?;

    // Establish WebSocket connection
    let (ws_stream, _) = connect_async(url).await?;

    // Wait for welcome message
    let welcome = wait_for_welcome(ws_stream, Duration::from_secs(10)).await?;

    // Now ready for subscriptions
    Ok(())
}
```

## Error Scenarios

### Token Expired
**Symptom**: Connection fails or gets rejected
**Solution**: Request new token and reconnect

### No Welcome Message
**Symptom**: Connected but no welcome within 10 seconds
**Solution**: Close connection and retry

### Invalid Token
**Symptom**: Connection immediately rejected
**Solution**: Request new token

### Server Unavailable
**Symptom**: Cannot connect to WebSocket server
**Solution**: Try next server in instance_servers list

## Best Practices

1. **Token Caching**
   - Cache token and timestamp
   - Refresh proactively before 24-hour expiration
   - Don't request token for every connection

2. **Connection Pooling**
   - Reuse connections when possible
   - Each connection can handle multiple subscriptions
   - Limit: 300 subscriptions per connection

3. **Heartbeat Management**
   - Send ping every 18 seconds (from server recommendation)
   - Expect pong within 10 seconds
   - Reconnect if pong not received

4. **Error Recovery**
   - On connection error: Get new token and reconnect
   - On subscription error: Retry with backoff
   - On data parsing error: Log but continue

5. **Failover**
   - Try all servers in instance_servers before giving up
   - Implement exponential backoff between retries

## Security Considerations

1. **Token Security**
   - Tokens are sensitive (especially private tokens)
   - Don't log tokens
   - Don't expose tokens in URLs (except WebSocket connection)

2. **Credential Protection**
   - API secret is used to sign requests
   - Never expose in logs or errors
   - Use zeroize for secure cleanup

3. **Passphrase Encryption**
   - Kucoin requires signed passphrase (version 2)
   - Passphrase signature = base64(hmac-sha256(secret, passphrase))
   - Send as `KC-API-PASSPHRASE` header with `KC-API-KEY-VERSION: 2`

## Performance Optimization

1. **Token Reuse**
   - Single token can be used for multiple connections
   - Cache and share across data/execution clients

2. **Connection Management**
   - Maintain persistent connections
   - Reconnect only on failure
   - Use connection pooling for multiple instruments

3. **Message Batching**
   - Subscribe to multiple channels in single message
   - Batch order submissions when possible

## Testing Strategy

### Unit Tests
```rust
#[test]
async fn test_get_public_token() {
    let client = KucoinHttpClient::new(...);
    let token = client.get_public_ws_token().await.unwrap();
    assert!(!token.token.is_empty());
    assert!(!token.instance_servers.is_empty());
}
```

### Integration Tests
```rust
#[test]
async fn test_websocket_connection_flow() {
    // 1. Get token
    let token = client.get_public_ws_token().await.unwrap();

    // 2. Connect with token
    let url = format!("{}?token={}", token.instance_servers[0].endpoint, token.token);
    let ws = connect_async(url).await.unwrap();

    // 3. Verify welcome message
    let welcome = receive_welcome(ws).await.unwrap();
    assert_eq!(welcome.msg_type, "welcome");
}
```

## Conclusion

Kucoin's token-based WebSocket system adds complexity but provides benefits:
- **Security**: Tokens can be revoked/expired
- **Load Balancing**: Multiple servers for failover
- **Flexibility**: Different tokens for different permission levels

The implementation in NautilusTrader handles this complexity transparently, providing a clean interface while managing tokens, connections, and reconnections automatically.

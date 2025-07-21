# Client API Reference

Complete reference for the rustycord Client API.

## Overview

The `Client` struct is the core HTTP client for interacting with the Discord REST API. It handles authentication, rate limiting, and provides methods for all Discord API endpoints.

## Client Structure

```rust
use rustycord::client::Client;
use rustycord::models::{User, Message, Guild, Channel};

pub struct Client {
    token: String,
    base_url: String,
    user_agent: String,
    http_client: reqwest::Client,
}
```

## Creating a Client

### Basic Client
```rust
use rustycord::Client;

let client = Client::new();
```

### Client with Custom Configuration
```rust
use rustycord::Client;

let client = Client::builder()
    .user_agent("MyBot/1.0")
    .timeout(std::time::Duration::from_secs(30))
    .build();
```

## Authentication

### Login
```rust
use rustycord::models::UserResponse;

let mut client = Client::new();
let user_response: UserResponse = client.login(token).await?;

println!("Logged in as: {}", user_response.username);
println!("Bot ID: {}", user_response.id);
```

### Get Current User
```rust
let current_user = client.get_current_user().await?;
println!("Current user: {:#?}", current_user);
```

## Message Operations

### Send Message
```rust
use rustycord::models::CreateMessage;

// Simple text message
let message = client.send_message("channel_id", "Hello, world!").await?;

// Message with embed
let embed = rustycord::embeds::Embed::new()
    .title("Test Embed")
    .description("This is a test embed")
    .color(0x00ff00);

let create_message = CreateMessage::new()
    .content("Message with embed")
    .embed(embed);

let message = client.send_message_with_options("channel_id", create_message).await?;
```

### Edit Message
```rust
let edited_message = client.edit_message(
    "channel_id",
    "message_id", 
    "Updated message content"
).await?;
```

### Delete Message
```rust
client.delete_message("channel_id", "message_id").await?;
```

### Get Message
```rust
let message = client.get_message("channel_id", "message_id").await?;
println!("Message: {}", message.content);
```

### Get Channel Messages
```rust
use rustycord::models::GetMessagesOptions;

// Get last 10 messages
let messages = client.get_channel_messages("channel_id", None).await?;

// Get messages with options
let options = GetMessagesOptions::new()
    .limit(50)
    .before("message_id");

let messages = client.get_channel_messages("channel_id", Some(options)).await?;
```

## Channel Operations

### Get Channel
```rust
let channel = client.get_channel("channel_id").await?;
println!("Channel name: {}", channel.name.unwrap_or("Unknown".to_string()));
```

### Create Channel
```rust
use rustycord::models::{CreateChannel, ChannelType};

let create_channel = CreateChannel::new()
    .name("new-channel")
    .channel_type(ChannelType::GuildText)
    .topic("Channel topic");

let channel = client.create_guild_channel("guild_id", create_channel).await?;
```

### Modify Channel
```rust
use rustycord::models::ModifyChannel;

let modify_channel = ModifyChannel::new()
    .name("updated-channel-name")
    .topic("Updated topic");

let channel = client.modify_channel("channel_id", modify_channel).await?;
```

### Delete Channel
```rust
client.delete_channel("channel_id").await?;
```

## Guild Operations

### Get Guild
```rust
let guild = client.get_guild("guild_id").await?;
println!("Guild name: {}", guild.name);
println!("Member count: {}", guild.member_count.unwrap_or(0));
```

### Get Guild Channels
```rust
let channels = client.get_guild_channels("guild_id").await?;
for channel in channels {
    println!("Channel: {}", channel.name.unwrap_or("Unknown".to_string()));
}
```

### Get Guild Members
```rust
use rustycord::models::GetGuildMembersOptions;

// Get all members (paginated)
let members = client.get_guild_members("guild_id", None).await?;

// Get members with options
let options = GetGuildMembersOptions::new()
    .limit(100)
    .after("user_id");

let members = client.get_guild_members("guild_id", Some(options)).await?;
```

### Get Guild Member
```rust
let member = client.get_guild_member("guild_id", "user_id").await?;
println!("Member: {}", member.user.username);
```

## User Operations

### Get User
```rust
let user = client.get_user("user_id").await?;
println!("Username: {}", user.username);
println!("Discriminator: {}", user.discriminator);
```

### Create DM Channel
```rust
let dm_channel = client.create_dm("user_id").await?;
println!("DM Channel ID: {}", dm_channel.id);
```

## Role Operations

### Get Guild Roles
```rust
let roles = client.get_guild_roles("guild_id").await?;
for role in roles {
    println!("Role: {} ({})", role.name, role.id);
}
```

### Create Role
```rust
use rustycord::models::CreateRole;

let create_role = CreateRole::new()
    .name("New Role")
    .color(0xff0000)
    .hoist(true)
    .mentionable(true);

let role = client.create_guild_role("guild_id", create_role).await?;
```

### Modify Role
```rust
use rustycord::models::ModifyRole;

let modify_role = ModifyRole::new()
    .name("Updated Role")
    .color(0x00ff00);

let role = client.modify_guild_role("guild_id", "role_id", modify_role).await?;
```

### Delete Role
```rust
client.delete_guild_role("guild_id", "role_id").await?;
```

### Add Role to Member
```rust
client.add_guild_member_role("guild_id", "user_id", "role_id").await?;
```

### Remove Role from Member
```rust
client.remove_guild_member_role("guild_id", "user_id", "role_id").await?;
```

## Reaction Operations

### Add Reaction
```rust
// Unicode emoji
client.create_reaction("channel_id", "message_id", "👍").await?;

// Custom emoji
client.create_reaction("channel_id", "message_id", "custom_emoji:123456789").await?;
```

### Remove Reaction
```rust
// Remove own reaction
client.delete_own_reaction("channel_id", "message_id", "👍").await?;

// Remove user's reaction
client.delete_user_reaction("channel_id", "message_id", "👍", "user_id").await?;

// Remove all reactions
client.delete_all_reactions("channel_id", "message_id").await?;
```

### Get Reactions
```rust
let users = client.get_reactions("channel_id", "message_id", "👍").await?;
for user in users {
    println!("User who reacted: {}", user.username);
}
```

## Error Handling

### Client Errors
```rust
use rustycord::client::{ClientError, ClientResult};

match client.send_message("channel_id", "Hello").await {
    Ok(message) => println!("Message sent: {}", message.id),
    Err(ClientError::Http(status, body)) => {
        eprintln!("HTTP Error {}: {}", status, body);
    },
    Err(ClientError::RateLimit(retry_after)) => {
        eprintln!("Rate limited. Retry after: {}s", retry_after);
    },
    Err(ClientError::Network(e)) => {
        eprintln!("Network error: {}", e);
    },
    Err(ClientError::Json(e)) => {
        eprintln!("JSON parsing error: {}", e);
    },
}
```

### Rate Limiting
```rust
use rustycord::client::RateLimitInfo;

// The client automatically handles rate limits, but you can check status
let rate_limit_info = client.get_rate_limit_info("endpoint").await?;
println!("Remaining requests: {}", rate_limit_info.remaining);
println!("Reset time: {}", rate_limit_info.reset_after);
```

## Advanced Usage

### Custom HTTP Headers
```rust
use rustycord::client::ClientBuilder;

let client = ClientBuilder::new()
    .header("X-Custom-Header", "value")
    .build();
```

### Proxy Support
```rust
let client = ClientBuilder::new()
    .proxy("http://proxy.example.com:8080")
    .build();
```

### Timeout Configuration
```rust
let client = ClientBuilder::new()
    .timeout(std::time::Duration::from_secs(60))
    .connect_timeout(std::time::Duration::from_secs(10))
    .build();
```

## Async Patterns

### Concurrent Requests
```rust
use futures::future::join_all;

let message_futures = vec![
    client.send_message("channel1", "Hello 1"),
    client.send_message("channel2", "Hello 2"),
    client.send_message("channel3", "Hello 3"),
];

let results = join_all(message_futures).await;
for result in results {
    match result {
        Ok(message) => println!("Sent message: {}", message.id),
        Err(e) => eprintln!("Failed to send message: {}", e),
    }
}
```

### Stream Processing
```rust
use futures::stream::{self, StreamExt};

let channel_ids = vec!["channel1", "channel2", "channel3"];
let client = std::sync::Arc::new(client);

let results: Vec<_> = stream::iter(channel_ids)
    .map(|channel_id| {
        let client = client.clone();
        async move {
            client.get_channel_messages(channel_id, None).await
        }
    })
    .buffer_unordered(3) // Process up to 3 requests concurrently
    .collect()
    .await;
```

## Testing

### Mock Client
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rustycord::client::MockClient;

    #[tokio::test]
    async fn test_send_message() {
        let mut mock_client = MockClient::new();
        
        mock_client
            .expect_send_message()
            .with("channel_id", "Hello")
            .returning(|_, _| Ok(Message::default()));
        
        let result = mock_client.send_message("channel_id", "Hello").await;
        assert!(result.is_ok());
    }
}
```

## Examples

### Bot Initialization with Client
```rust
use rustycord::{Client, Bot};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_TOKEN")?;
    
    // Create and configure client
    let mut client = Client::new();
    let user_response = client.login(token).await?;
    
    println!("Bot logged in as: {}", user_response.username);
    
    // Use client directly or with Bot
    let mut bot = Bot::with_client(client);
    bot.start().await?;
    
    Ok(())
}
```

### Message Processing Pipeline
```rust
async fn process_messages(client: &Client, channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get recent messages
    let messages = client.get_channel_messages(channel_id, None).await?;
    
    // Process each message
    for message in messages {
        if message.content.contains("spam") {
            // Delete spam message
            client.delete_message(channel_id, &message.id).await?;
            
            // Send warning to user
            let dm_channel = client.create_dm(&message.author.id).await?;
            client.send_message(&dm_channel.id, "Please don't spam!").await?;
        }
    }
    
    Ok(())
}
```

## API Reference Summary

| Method | Description | Returns |
|--------|-------------|---------|
| `Client::new()` | Create new client | `Client` |
| `client.login(token)` | Authenticate with Discord | `UserResponse` |
| `client.send_message(channel, content)` | Send text message | `Message` |
| `client.get_channel(id)` | Get channel info | `Channel` |
| `client.get_guild(id)` | Get guild info | `Guild` |
| `client.get_user(id)` | Get user info | `User` |
| `client.create_reaction(channel, message, emoji)` | Add reaction | `()` |
| `client.get_guild_members(guild, options)` | Get guild members | `Vec<Member>` |

## Rate Limits

The Discord API has rate limits that the client automatically handles:

- **Global Rate Limit**: 50 requests per second
- **Per-Route Rate Limits**: Vary by endpoint
- **Per-Guild Rate Limits**: Apply to guild-specific operations

The client will automatically retry requests that hit rate limits with appropriate backoff.

## Related Documentation

- [Bot Basics](../user-guide/bot-basics.md) - Using the client with bots
- [Message Handler Example](../examples/message-handler.md) - Client usage in handlers
- [Error Handling Guide](../user-guide/error-handling.md) - Handling client errors

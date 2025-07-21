# Message Handler Example

This example shows how to create custom message handlers for more complex bot interactions.

## Overview

Message handlers in rustycord allow you to create modular, reusable components that process Discord messages. Each handler implements the `MessageHandler` trait and can be registered with your bot.

## Basic Message Handler

```rust
use rustycord::{Bot, MessageHandler, logger};
use rustycord::models::Message;
use async_trait::async_trait;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger("info".to_string())?;
    
    let token = std::env::var("DISCORD_TOKEN")?;
    let mut bot = Bot::new(None).await;
    
    bot.login(token).await;
    
    // Register multiple handlers
    bot.register_message_handler(Box::new(MathHandler));
    bot.register_message_handler(Box::new(InfoHandler));
    bot.register_message_handler(Box::new(ModeratorHandler));
    
    bot.start().await?;
    Ok(())
}
```

## Math Handler

A handler that performs basic math operations:

```rust
struct MathHandler;

#[async_trait]
impl MessageHandler for MathHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content.starts_with("!math ") {
            let expression = message.content.strip_prefix("!math ").unwrap_or("");
            
            match evaluate_expression(expression) {
                Ok(result) => Ok(Some(format!("🧮 Result: {}", result))),
                Err(e) => Ok(Some(format!("❌ Error: {}", e)))
            }
        } else {
            Ok(None)
        }
    }
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // Simple expression evaluator (you might want to use a proper parser)
    let parts: Vec<&str> = expr.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err("Usage: !math <number> <operator> <number>".to_string());
    }
    
    let a: f64 = parts[0].parse().map_err(|_| "Invalid first number")?;
    let op = parts[1];
    let b: f64 = parts[2].parse().map_err(|_| "Invalid second number")?;
    
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        },
        "%" => Ok(a % b),
        "^" => Ok(a.powf(b)),
        _ => Err("Supported operators: +, -, *, /, %, ^".to_string())
    }
}
```

## Info Handler

A handler that provides server and user information:

```rust
struct InfoHandler;

#[async_trait]
impl MessageHandler for InfoHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        match message.content.as_str() {
            "!info server" => {
                Ok(Some(format!(
                    "🏠 **Server Information**\n\
                     Guild ID: {}\n\
                     Channel ID: {}",
                    message.guild_id.unwrap_or_default(),
                    message.channel_id
                )))
            },
            "!info user" => {
                Ok(Some(format!(
                    "👤 **User Information**\n\
                     Username: {}\n\
                     User ID: {}\n\
                     Mention: <@{}>",
                    message.author.username,
                    message.author.id,
                    message.author.id
                )))
            },
            "!help" => {
                Ok(Some(
                    "📚 **Available Commands**\n\
                     `!info server` - Server information\n\
                     `!info user` - Your user information\n\
                     `!math <a> <op> <b>` - Basic math operations\n\
                     `!help` - Show this help message".to_string()
                ))
            },
            _ => Ok(None)
        }
    }
}
```

## Stateful Handler

A handler that maintains state between messages:

```rust
use std::sync::{Arc, Mutex};

struct CounterHandler {
    counters: Arc<Mutex<HashMap<String, u32>>>,
}

impl CounterHandler {
    fn new() -> Self {
        Self {
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MessageHandler for CounterHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content.starts_with("!count") {
            let mut counters = self.counters.lock().unwrap();
            let user_id = message.author.id.to_string();
            
            let count = counters.entry(user_id.clone()).or_insert(0);
            *count += 1;
            
            Ok(Some(format!("📊 {}, your message count: {}", message.author.username, count)))
        } else {
            Ok(None)
        }
    }
}
```

## Moderator Handler

A handler with permission checks:

```rust
struct ModeratorHandler;

#[async_trait]
impl MessageHandler for ModeratorHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content.starts_with("!mod ") {
            // Note: In a real implementation, you'd check actual Discord permissions
            if !self.is_moderator(&message.author) {
                return Ok(Some("❌ You don't have permission to use moderator commands.".to_string()));
            }
            
            let command = message.content.strip_prefix("!mod ").unwrap_or("");
            
            match command {
                "clean" => {
                    Ok(Some("🧹 Cleaned up recent messages.".to_string()))
                },
                "warn" => {
                    Ok(Some("⚠️ Warning issued.".to_string()))
                },
                _ => {
                    Ok(Some("❓ Unknown moderator command. Available: clean, warn".to_string()))
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl ModeratorHandler {
    fn is_moderator(&self, user: &rustycord::models::User) -> bool {
        // Simplified check - in reality, you'd check Discord roles/permissions
        // For demo purposes, we'll check if the username contains "mod"
        user.username.to_lowercase().contains("mod")
    }
}
```

## Handler Registration

```rust
// Register handlers in your main function
async fn setup_bot() -> Result<Bot, Box<dyn std::error::Error>> {
    let mut bot = Bot::new(None).await;
    
    // Basic handlers
    bot.register_message_handler(Box::new(MathHandler));
    bot.register_message_handler(Box::new(InfoHandler));
    bot.register_message_handler(Box::new(ModeratorHandler));
    
    // Stateful handler
    bot.register_message_handler(Box::new(CounterHandler::new()));
    
    Ok(bot)
}
```

## Best Practices

1. **Keep handlers focused**: Each handler should have a single responsibility
2. **Use async operations**: Make database calls or HTTP requests async
3. **Handle errors gracefully**: Return meaningful error messages to users
4. **Validate input**: Always validate user input before processing
5. **Use proper permissions**: Check user permissions for sensitive commands
6. **State management**: Use `Arc<Mutex<T>>` for shared state between handler instances

## Advanced Examples

- [Embeds Example](embeds.md) - Rich embed responses
- [Database Integration](../user-guide/database.md) - Persistent data storage
- [API Integration](../user-guide/external-apis.md) - Calling external APIs

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rustycord::models::{User, Message};

    #[tokio::test]
    async fn test_math_handler() {
        let handler = MathHandler;
        let message = Message {
            content: "!math 5 + 3".to_string(),
            author: User {
                id: "123".to_string(),
                username: "testuser".to_string(),
            },
            // ... other fields
        };
        
        let result = handler.handle_message(&message).await.unwrap();
        assert_eq!(result, Some("🧮 Result: 8".to_string()));
    }
}
```

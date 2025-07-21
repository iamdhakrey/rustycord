# Bot Basics

Learn the fundamental concepts and patterns for building Discord bots with rustycord.

## Overview

This guide covers the essential concepts you need to understand when building Discord bots with rustycord, including bot lifecycle, event handling, permissions, and best practices.

## Bot Architecture

```rust
use rustycord::{Bot, Client, MessageHandler, logger};
use rustycord::models::{Message, User};
use rustycord::gateway::intents::Intents;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    logger::setup_logger("info".to_string())?;
    
    // Create bot with specific intents
    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
    let mut bot = Bot::new(Some(intents.bits() as i32)).await;
    
    // Login
    let token = std::env::var("DISCORD_TOKEN")?;
    bot.login(token).await;
    
    // Register handlers
    bot.register_message_handler(Box::new(BasicCommandHandler));
    bot.register_message_handler(Box::new(AdminHandler::new()));
    
    // Start the bot
    bot.start().await?;
    
    Ok(())
}
```

## Bot Lifecycle

### 1. Initialization
```rust
// Create bot instance
let mut bot = Bot::new(None).await;

// Configure bot settings
bot.set_activity("Watching for commands");
bot.set_status("online");
```

### 2. Authentication
```rust
// Login with bot token
let user_response = bot.login(token).await;
log::info!("Bot logged in as: {}", user_response.username);
```

### 3. Handler Registration
```rust
// Register message handlers
bot.register_message_handler(Box::new(CommandHandler));
bot.register_message_handler(Box::new(ModerationHandler));

// Register event handlers (future feature)
// bot.register_event_handler(Box::new(GuildHandler));
```

### 4. Event Loop
```rust
// Start the bot (blocking call)
bot.start().await?;
```

## Message Handling Patterns

### Basic Command Handler
```rust
struct BasicCommandHandler;

#[async_trait]
impl MessageHandler for BasicCommandHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        // Ignore bot messages
        if message.author.bot.unwrap_or(false) {
            return Ok(None);
        }
        
        // Parse command
        let content = message.content.trim();
        if !content.starts_with('!') {
            return Ok(None);
        }
        
        let parts: Vec<&str> = content[1..].split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }
        
        let command = parts[0].to_lowercase();
        let args = &parts[1..];
        
        match command.as_str() {
            "ping" => Ok(Some("🏓 Pong!".to_string())),
            "help" => Ok(Some(self.get_help_text())),
            "echo" => {
                if args.is_empty() {
                    Ok(Some("❌ Please provide text to echo.".to_string()))
                } else {
                    Ok(Some(args.join(" ")))
                }
            },
            "info" => Ok(Some(self.get_bot_info())),
            _ => Ok(None), // Command not handled by this handler
        }
    }
}

impl BasicCommandHandler {
    fn get_help_text(&self) -> String {
        "📚 **Available Commands**\n\
         `!ping` - Test bot responsiveness\n\
         `!help` - Show this help message\n\
         `!echo <text>` - Echo your message\n\
         `!info` - Show bot information".to_string()
    }
    
    fn get_bot_info(&self) -> String {
        "🤖 **Bot Information**\n\
         Name: RustyCord Bot\n\
         Version: 0.1.1\n\
         Language: Rust\n\
         Library: RustyCord".to_string()
    }
}
```

### Advanced Command Handler with Subcommands
```rust
use std::collections::HashMap;

struct AdvancedCommandHandler {
    commands: HashMap<String, Box<dyn Command + Send + Sync>>,
}

#[async_trait]
trait Command {
    async fn execute(&self, args: &[&str], message: &Message) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    fn description(&self) -> &str;
    fn usage(&self) -> &str;
}

impl AdvancedCommandHandler {
    fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command + Send + Sync>> = HashMap::new();
        
        commands.insert("user".to_string(), Box::new(UserCommand));
        commands.insert("server".to_string(), Box::new(ServerCommand));
        commands.insert("math".to_string(), Box::new(MathCommand));
        
        Self { commands }
    }
}

#[async_trait]
impl MessageHandler for AdvancedCommandHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if !message.content.starts_with('!') {
            return Ok(None);
        }
        
        let parts: Vec<&str> = message.content[1..].split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }
        
        let command_name = parts[0].to_lowercase();
        let args = &parts[1..];
        
        if let Some(command) = self.commands.get(&command_name) {
            match command.execute(args, message).await {
                Ok(response) => Ok(Some(response)),
                Err(e) => Ok(Some(format!("❌ Error executing command: {}", e))),
            }
        } else if command_name == "help" {
            Ok(Some(self.generate_help()))
        } else {
            Ok(None)
        }
    }
}

impl AdvancedCommandHandler {
    fn generate_help(&self) -> String {
        let mut help = "📚 **Available Commands**\n".to_string();
        
        for (name, command) in &self.commands {
            help.push_str(&format!(
                "`!{} {}` - {}\n",
                name,
                command.usage(),
                command.description()
            ));
        }
        
        help.push_str("`!help` - Show this help message");
        help
    }
}

// Command implementations
struct UserCommand;

#[async_trait]
impl Command for UserCommand {
    async fn execute(&self, args: &[&str], message: &Message) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match args.get(0) {
            Some(&"info") => {
                Ok(format!(
                    "👤 **User Information**\n\
                     Username: {}\n\
                     User ID: {}\n\
                     Account Type: {}",
                    message.author.username,
                    message.author.id,
                    if message.author.bot.unwrap_or(false) { "Bot" } else { "User" }
                ))
            },
            Some(&"avatar") => {
                Ok(format!(
                    "🖼️ **User Avatar**\n\
                     Username: {}\n\
                     Avatar URL: {}",
                    message.author.username,
                    message.author.avatar_url().unwrap_or("No avatar".to_string())
                ))
            },
            _ => Ok("❓ Available subcommands: `info`, `avatar`".to_string()),
        }
    }
    
    fn description(&self) -> &str {
        "User-related commands"
    }
    
    fn usage(&self) -> &str {
        "<info|avatar>"
    }
}

struct ServerCommand;

#[async_trait]
impl Command for ServerCommand {
    async fn execute(&self, args: &[&str], message: &Message) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match args.get(0) {
            Some(&"info") => {
                Ok(format!(
                    "🏠 **Server Information**\n\
                     Guild ID: {}\n\
                     Channel ID: {}",
                    message.guild_id.unwrap_or_default(),
                    message.channel_id
                ))
            },
            _ => Ok("❓ Available subcommands: `info`".to_string()),
        }
    }
    
    fn description(&self) -> &str {
        "Server-related commands"
    }
    
    fn usage(&self) -> &str {
        "<info>"
    }
}

struct MathCommand;

#[async_trait]
impl Command for MathCommand {
    async fn execute(&self, args: &[&str], message: &Message) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if args.len() != 3 {
            return Ok("❓ Usage: `!math <number> <operator> <number>`".to_string());
        }
        
        let a: f64 = args[0].parse().map_err(|_| "Invalid first number")?;
        let op = args[1];
        let b: f64 = args[2].parse().map_err(|_| "Invalid second number")?;
        
        let result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return Ok("❌ Division by zero".to_string());
                }
                a / b
            },
            "%" => a % b,
            "^" => a.powf(b),
            _ => return Ok("❌ Supported operators: +, -, *, /, %, ^".to_string()),
        };
        
        Ok(format!("🧮 {} {} {} = {}", a, op, b, result))
    }
    
    fn description(&self) -> &str {
        "Basic math operations"
    }
    
    fn usage(&self) -> &str {
        "<number> <operator> <number>"
    }
}
```

## Permission Handling

```rust
use rustycord::models::{User, Role, Permission};

struct PermissionChecker;

impl PermissionChecker {
    fn has_permission(user: &User, required_permission: Permission) -> bool {
        // In a real implementation, you'd check Discord permissions
        // This is a simplified example
        user.roles.iter().any(|role| role.permissions.contains(required_permission))
    }
    
    fn is_admin(user: &User) -> bool {
        Self::has_permission(user, Permission::ADMINISTRATOR)
    }
    
    fn is_moderator(user: &User) -> bool {
        Self::has_permission(user, Permission::MANAGE_MESSAGES) ||
        Self::has_permission(user, Permission::KICK_MEMBERS) ||
        Self::is_admin(user)
    }
    
    fn can_manage_roles(user: &User) -> bool {
        Self::has_permission(user, Permission::MANAGE_ROLES) || Self::is_admin(user)
    }
}

// Usage in command handler
struct AdminHandler;

#[async_trait]
impl MessageHandler for AdminHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if !message.content.starts_with("!admin ") {
            return Ok(None);
        }
        
        // Check if user has admin permissions
        if !PermissionChecker::is_admin(&message.author) {
            return Ok(Some("❌ You don't have permission to use admin commands.".to_string()));
        }
        
        let command = message.content.strip_prefix("!admin ").unwrap_or("");
        
        match command {
            "shutdown" => {
                log::warn!("🛑 Admin {} requested bot shutdown", message.author.username);
                Ok(Some("🛑 Shutting down bot...".to_string()))
            },
            "restart" => {
                log::info!("🔄 Admin {} requested bot restart", message.author.username);
                Ok(Some("🔄 Restarting bot...".to_string()))
            },
            "stats" => {
                Ok(Some(self.get_bot_stats()))
            },
            _ => Ok(Some("❓ Available admin commands: shutdown, restart, stats".to_string())),
        }
    }
}

impl AdminHandler {
    fn get_bot_stats(&self) -> String {
        "📊 **Bot Statistics**\n\
         Uptime: 2 hours, 34 minutes\n\
         Messages Processed: 1,247\n\
         Commands Executed: 89\n\
         Memory Usage: 45.2 MB\n\
         Active Connections: 1".to_string()
    }
}
```

## Error Handling

```rust
use anyhow::{Context, Result};

#[async_trait]
impl MessageHandler for ErrorHandlingHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if !message.content.starts_with("!test") {
            return Ok(None);
        }
        
        match self.process_test_command(message).await {
            Ok(response) => Ok(Some(response)),
            Err(e) => {
                log::error!("❌ Error processing test command: {:#}", e);
                Ok(Some("❌ An error occurred while processing your command.".to_string()))
            }
        }
    }
}

impl ErrorHandlingHandler {
    async fn process_test_command(&self, message: &Message) -> Result<String> {
        // Simulate potential failure points
        self.validate_user(message)
            .context("User validation failed")?;
        
        let data = self.fetch_data()
            .await
            .context("Failed to fetch required data")?;
        
        let processed = self.process_data(data)
            .context("Data processing failed")?;
        
        Ok(format!("✅ Command completed: {}", processed))
    }
    
    fn validate_user(&self, message: &Message) -> Result<()> {
        if message.author.username.is_empty() {
            anyhow::bail!("Username cannot be empty");
        }
        Ok(())
    }
    
    async fn fetch_data(&self) -> Result<String> {
        // Simulate async operation that might fail
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        Ok("sample_data".to_string())
    }
    
    fn process_data(&self, data: String) -> Result<String> {
        if data.is_empty() {
            anyhow::bail!("Cannot process empty data");
        }
        Ok(format!("processed_{}", data))
    }
}
```

## Best Practices

### 1. Handler Organization
```rust
// Organize handlers by functionality
mod handlers {
    pub mod commands;
    pub mod moderation;
    pub mod utility;
    pub mod fun;
}

// Register handlers in a organized way
fn register_handlers(bot: &mut Bot) {
    // Core functionality
    bot.register_message_handler(Box::new(handlers::commands::BasicCommands));
    bot.register_message_handler(Box::new(handlers::utility::UtilityCommands));
    
    // Moderation (order matters - check permissions first)
    bot.register_message_handler(Box::new(handlers::moderation::ModerationCommands));
    
    // Fun commands (last, as they might be catch-all)
    bot.register_message_handler(Box::new(handlers::fun::FunCommands));
}
```

### 2. Configuration Management
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct BotConfig {
    pub token: String,
    pub prefix: String,
    pub log_level: String,
    pub owner_id: String,
    pub admin_roles: Vec<String>,
    pub disabled_commands: Vec<String>,
}

impl BotConfig {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("config.toml")?;
        let config: BotConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}
```

### 3. Rate Limiting
```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct RateLimiter {
    user_last_command: Arc<Mutex<HashMap<String, Instant>>>,
    cooldown: Duration,
}

impl RateLimiter {
    fn new(cooldown_seconds: u64) -> Self {
        Self {
            user_last_command: Arc::new(Mutex::new(HashMap::new())),
            cooldown: Duration::from_secs(cooldown_seconds),
        }
    }
    
    fn check_rate_limit(&self, user_id: &str) -> bool {
        let mut last_commands = self.user_last_command.lock().unwrap();
        let now = Instant::now();
        
        if let Some(last_time) = last_commands.get(user_id) {
            if now.duration_since(*last_time) < self.cooldown {
                return false; // Rate limited
            }
        }
        
        last_commands.insert(user_id.to_string(), now);
        true // Allow command
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rustycord::models::{User, Message};

    fn create_test_message(content: &str, username: &str) -> Message {
        Message {
            content: content.to_string(),
            author: User {
                id: "123456789".to_string(),
                username: username.to_string(),
                bot: Some(false),
                // ... other fields
            },
            // ... other fields
        }
    }

    #[tokio::test]
    async fn test_basic_command_handler() {
        let handler = BasicCommandHandler;
        let message = create_test_message("!ping", "testuser");
        
        let result = handler.handle_message(&message).await.unwrap();
        assert_eq!(result, Some("🏓 Pong!".to_string()));
    }

    #[tokio::test]
    async fn test_echo_command() {
        let handler = BasicCommandHandler;
        let message = create_test_message("!echo Hello World", "testuser");
        
        let result = handler.handle_message(&message).await.unwrap();
        assert_eq!(result, Some("Hello World".to_string()));
    }

    #[tokio::test]
    async fn test_non_command_message() {
        let handler = BasicCommandHandler;
        let message = create_test_message("Hello there", "testuser");
        
        let result = handler.handle_message(&message).await.unwrap();
        assert_eq!(result, None);
    }
}
```

## Related Documentation

- [Installation Guide](../getting-started/installation.md) - Set up your development environment
- [Prefix Commands](prefix-commands.md) - Advanced command patterns
- [Examples](../examples/basic-bot.md) - Working code examples

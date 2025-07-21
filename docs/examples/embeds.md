# Embeds Example

Learn how to create rich, interactive embeds for your Discord bot using rustycord.

## Overview

Discord embeds are rich message components that can display formatted text, images, fields, and other interactive elements. rustycord provides a comprehensive embed system for creating beautiful bot responses.

## Basic Embed

```rust
use rustycord::{Bot, MessageHandler, embeds::Embed, embeds::EmbedField};
use rustycord::models::Message;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_TOKEN")?;
    let mut bot = Bot::new(None).await;
    
    bot.login(token).await;
    bot.register_message_handler(Box::new(EmbedHandler));
    
    bot.start().await?;
    Ok(())
}

struct EmbedHandler;

#[async_trait]
impl MessageHandler for EmbedHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        match message.content.as_str() {
            "!embed basic" => {
                let embed = Embed::new()
                    .title("Hello, World!")
                    .description("This is a basic embed example.")
                    .color(0x00ff00); // Green color
                
                // Send embed (simplified - actual implementation may vary)
                Ok(Some(format!("Embed: {}", embed.to_json())))
            },
            _ => Ok(None)
        }
    }
}
```

## Rich Embed Examples

### Info Embed

```rust
fn create_info_embed() -> Embed {
    Embed::new()
        .title("📊 Server Statistics")
        .description("Current server information and statistics")
        .color(0x3498db) // Blue
        .thumbnail("https://example.com/server-icon.png")
        .add_field(EmbedField::new("Total Members", "1,234", true))
        .add_field(EmbedField::new("Online Members", "567", true))
        .add_field(EmbedField::new("Text Channels", "15", true))
        .add_field(EmbedField::new("Voice Channels", "8", true))
        .add_field(EmbedField::new("Server Boost Level", "Level 2", true))
        .add_field(EmbedField::new("Server Created", "2020-01-15", true))
        .footer("Server ID: 123456789012345678")
        .timestamp()
}
```

### User Profile Embed

```rust
fn create_user_profile_embed(message: &Message) -> Embed {
    Embed::new()
        .title(format!("👤 User Profile: {}", message.author.username))
        .description("User information and statistics")
        .color(0xe91e63) // Pink
        .thumbnail(&message.author.avatar_url().unwrap_or_default())
        .add_field(EmbedField::new("User ID", &message.author.id, false))
        .add_field(EmbedField::new("Account Created", "2019-03-15", true))
        .add_field(EmbedField::new("Joined Server", "2020-06-20", true))
        .add_field(EmbedField::new("Roles", "Member, Active User", false))
        .add_field(EmbedField::new("Messages Sent", "2,847", true))
        .add_field(EmbedField::new("Last Active", "Today at 2:30 PM", true))
        .footer(&format!("Requested by {}", message.author.username))
        .timestamp()
}
```

### Error Embed

```rust
fn create_error_embed(error_message: &str) -> Embed {
    Embed::new()
        .title("❌ Error")
        .description(error_message)
        .color(0xe74c3c) // Red
        .footer("If this error persists, contact an administrator")
        .timestamp()
}
```

### Success Embed

```rust
fn create_success_embed(success_message: &str) -> Embed {
    Embed::new()
        .title("✅ Success")
        .description(success_message)
        .color(0x2ecc71) // Green
        .footer("Operation completed successfully")
        .timestamp()
}
```

## Advanced Embed Handler

```rust
struct AdvancedEmbedHandler;

#[async_trait]
impl MessageHandler for AdvancedEmbedHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let parts: Vec<&str> = message.content.split_whitespace().collect();
        
        if parts.is_empty() || parts[0] != "!embed" {
            return Ok(None);
        }
        
        let embed = match parts.get(1) {
            Some(&"info") => self.create_server_info_embed(message),
            Some(&"profile") => self.create_user_profile_embed(message),
            Some(&"weather") => self.create_weather_embed().await?,
            Some(&"music") => self.create_music_embed(),
            Some(&"poll") => self.create_poll_embed(&parts[2..]),
            Some(&"help") => self.create_help_embed(),
            _ => self.create_default_embed(),
        };
        
        // In a real implementation, you'd send this embed through the Discord API
        Ok(Some(format!("Embed: {}", embed.to_json())))
    }
}

impl AdvancedEmbedHandler {
    fn create_server_info_embed(&self, message: &Message) -> Embed {
        Embed::new()
            .title("🏠 Server Information")
            .description("Detailed information about this Discord server")
            .color(0x7289da)
            .add_field(EmbedField::new("Server Name", "My Awesome Server", false))
            .add_field(EmbedField::new("Total Members", "1,234", true))
            .add_field(EmbedField::new("Online Now", "567", true))
            .add_field(EmbedField::new("Text Channels", "25", true))
            .add_field(EmbedField::new("Voice Channels", "12", true))
            .add_field(EmbedField::new("Roles", "15", true))
            .add_field(EmbedField::new("Emojis", "89", true))
            .add_field(EmbedField::new("Server Owner", "<@123456789>", false))
            .add_field(EmbedField::new("Created On", "January 15, 2020", false))
            .thumbnail("https://example.com/server-icon.png")
            .footer(&format!("Requested by {}", message.author.username))
            .timestamp()
    }
    
    fn create_user_profile_embed(&self, message: &Message) -> Embed {
        Embed::new()
            .title(format!("👤 {}'s Profile", message.author.username))
            .color(0x9b59b6)
            .add_field(EmbedField::new("Username", &message.author.username, true))
            .add_field(EmbedField::new("User ID", &message.author.id, true))
            .add_field(EmbedField::new("Status", "🟢 Online", true))
            .add_field(EmbedField::new("Join Date", "June 20, 2020", true))
            .add_field(EmbedField::new("Account Age", "3 years, 7 months", true))
            .add_field(EmbedField::new("Roles", "Member, Active User, Helper", false))
            .thumbnail(&message.author.avatar_url().unwrap_or_default())
            .footer("User Profile")
            .timestamp()
    }
    
    async fn create_weather_embed(&self) -> Result<Embed, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, you'd fetch weather data from an API
        Ok(Embed::new()
            .title("🌤️ Weather Forecast")
            .description("Current weather conditions")
            .color(0x87ceeb)
            .add_field(EmbedField::new("Location", "San Francisco, CA", false))
            .add_field(EmbedField::new("Temperature", "22°C (72°F)", true))
            .add_field(EmbedField::new("Condition", "Partly Cloudy", true))
            .add_field(EmbedField::new("Humidity", "65%", true))
            .add_field(EmbedField::new("Wind", "15 mph NW", true))
            .add_field(EmbedField::new("UV Index", "6 (High)", true))
            .add_field(EmbedField::new("Visibility", "10 km", true))
            .thumbnail("https://example.com/weather-icon.png")
            .footer("Weather data provided by OpenWeatherMap")
            .timestamp())
    }
    
    fn create_music_embed(&self) -> Embed {
        Embed::new()
            .title("🎵 Now Playing")
            .description("Current music queue status")
            .color(0xff6b6b)
            .add_field(EmbedField::new("Song", "Never Gonna Give You Up", false))
            .add_field(EmbedField::new("Artist", "Rick Astley", true))
            .add_field(EmbedField::new("Duration", "3:33", true))
            .add_field(EmbedField::new("Requested by", "<@123456789>", true))
            .add_field(EmbedField::new("Queue Position", "1 of 5", true))
            .add_field(EmbedField::new("Volume", "75%", true))
            .add_field(EmbedField::new("Loop", "Disabled", true))
            .thumbnail("https://example.com/album-art.jpg")
            .footer("Music Player | Use !music help for commands")
            .timestamp()
    }
    
    fn create_poll_embed(&self, options: &[&str]) -> Embed {
        let mut embed = Embed::new()
            .title("📊 Poll")
            .description("Vote by reacting with the corresponding emoji!")
            .color(0xf39c12);
        
        let emojis = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟"];
        
        for (i, option) in options.iter().take(10).enumerate() {
            embed = embed.add_field(EmbedField::new(
                &format!("{} Option {}", emojis[i], i + 1),
                option,
                false
            ));
        }
        
        embed
            .footer("React to vote! Poll ends in 24 hours.")
            .timestamp()
    }
    
    fn create_help_embed(&self) -> Embed {
        Embed::new()
            .title("📚 Embed Commands Help")
            .description("Available embed commands and their usage")
            .color(0x3498db)
            .add_field(EmbedField::new("!embed info", "Show server information", false))
            .add_field(EmbedField::new("!embed profile", "Show your user profile", false))
            .add_field(EmbedField::new("!embed weather", "Get weather information", false))
            .add_field(EmbedField::new("!embed music", "Show music player status", false))
            .add_field(EmbedField::new("!embed poll <options>", "Create a poll with options", false))
            .add_field(EmbedField::new("!embed help", "Show this help message", false))
            .footer("Use these commands to see different embed examples")
            .timestamp()
    }
    
    fn create_default_embed(&self) -> Embed {
        Embed::new()
            .title("🤖 RustyCord Bot")
            .description("Welcome to the embed system demonstration!")
            .color(0x7289da)
            .add_field(EmbedField::new("Available Commands", "Use `!embed help` to see all commands", false))
            .add_field(EmbedField::new("Bot Version", "0.1.1", true))
            .add_field(EmbedField::new("Language", "Rust", true))
            .add_field(EmbedField::new("Library", "RustyCord", true))
            .footer("RustyCord - Fast, Safe, Reliable")
            .timestamp()
    }
}
```

## Embed Builder Pattern

```rust
// Fluent interface for building embeds
let embed = Embed::new()
    .title("My Embed")
    .description("A description")
    .url("https://example.com")
    .color(0x00ff00)
    .author("Author Name", Some("https://example.com"), Some("https://example.com/icon.png"))
    .thumbnail("https://example.com/thumbnail.png")
    .image("https://example.com/image.png")
    .add_field(EmbedField::new("Field 1", "Value 1", true))
    .add_field(EmbedField::new("Field 2", "Value 2", true))
    .footer("Footer text")
    .timestamp();
```

## Best Practices

1. **Color Coding**: Use consistent colors for different types of messages
   - Success: Green (`0x2ecc71`)
   - Error: Red (`0xe74c3c`)
   - Info: Blue (`0x3498db`)
   - Warning: Orange (`0xf39c12`)

2. **Field Limits**: Discord embeds have limits:
   - Title: 256 characters
   - Description: 4096 characters
   - Fields: 25 fields maximum
   - Field name: 256 characters
   - Field value: 1024 characters

3. **Image Guidelines**:
   - Use HTTPS URLs for images
   - Keep image sizes reasonable
   - Provide fallback text for accessibility

4. **Timestamps**: Always include timestamps for time-sensitive information

5. **Footer Information**: Use footers for metadata and attribution

## Testing Embeds

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_creation() {
        let embed = Embed::new()
            .title("Test Embed")
            .description("Test Description")
            .color(0x00ff00);
        
        assert_eq!(embed.title(), Some("Test Embed"));
        assert_eq!(embed.description(), Some("Test Description"));
        assert_eq!(embed.color(), Some(0x00ff00));
    }
}
```

## Related Examples

- [Message Handler Example](message-handler.md) - Using embeds in message handlers
- [API Integration](../user-guide/external-apis.md) - Fetching data for embeds
- [User Guide](../user-guide/prefix-commands.md) - Command patterns with embeds

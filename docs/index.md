<div align="center">
  <img src="images/rustycord-logo.png" alt="rustycord Logo" width="128" height="128">
  
</div>
  
  [![Crates.io](https://img.shields.io/crates/v/rustycord.svg)](https://crates.io/crates/rustycord) [![Documentation](https://docs.rs/rustycord/badge.svg)](https://docs.rs/rustycord) [![dependency status](https://deps.rs/repo/github/iamdhakrey/rustycord/status.svg)](https://deps.rs/repo/github/iamdhakrey/rustycord) [![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/iamdhakrey/rustycord/blob/main/LICENSE) [![GitHub Stars](https://img.shields.io/github/stars/iamdhakrey/rustycord.svg)](https://github.com/iamdhakrey/rustycord/stargazers) [![GitHub Issues](https://img.shields.io/github/issues/iamdhakrey/rustycord.svg)](https://github.com/iamdhakrey/rustycord/issues) [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/iamdhakrey/rustycord.svg)](https://github.com/iamdhakrey/rustycord/pulls) [![Rust](https://img.shields.io/badge/rust-1.60%2B-orange.svg)](https://www.rust-lang.org) [![Discord API](https://img.shields.io/badge/discord%20api-v10-7289da.svg)](https://discord.com/developers/docs/) [![Downloads](https://img.shields.io/crates/d/rustycord.svg)](https://crates.io/crates/rustycord)
  

A fast, lightweight, and feature-rich Discord bot library written in Rust.

!!! warning "Development Notice"
    **RustyCord is currently in heavy development and is NOT ready for production use.**
    
    - APIs may change without notice
    - Features are incomplete and experimental
    - Breaking changes occur frequently
    - Documentation may be outdated
    - Use at your own risk for development/testing only
    
    **Do not use this library for production bots yet.** Wait for the stable 1.0 release.

!!! info "Current Status"
    - ✅ Basic bot functionality working
    - ✅ Message handling system implemented
    - ✅ Prefix command system functional
    - ⚠️ Advanced features under development
    - ❌ Slash commands not yet implemented
    - ❌ Voice support not available
    - ❌ Comprehensive testing needed

## Overview

rustycord is a modern Discord bot library that provides a clean, type-safe API for building Discord bots. Built with performance and reliability in mind, it offers comprehensive logging, flexible message handling, and an intuitive event system.

## Key Features

- **🚀 Async/Await Support**: Built on Tokio for high-performance async operations
- **📝 Comprehensive Logging**: 5-level logging system with console and file output
- **🔧 Flexible Message Handlers**: Trait-based message handling system
- **🎯 Type Safety**: Leverages Rust's type system for compile-time safety
- **📡 WebSocket Gateway**: Full Discord Gateway v10 support
- **🌐 HTTP Client**: Complete Discord REST API client
- **🎨 Rich Embeds**: Built-in support for Discord embeds
- **⚡ Event System**: Comprehensive event handling and dispatching

## Quick Start

```rust
use rustycord::{Bot, Client, MessageHandler, logger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    logger::setup_logger(logger::LogLevel::Info)?;
    
    // Create client and bot
    let token = std::env::var("DISCORD_TOKEN")?;
    let client = Client::new(&token).await?;
    let mut bot = Bot::new(client);
    
    // Register message handlers
    bot.register_message_handler(Box::new(EchoHandler));
    
    // Start the bot
    bot.start().await?;
    
    Ok(())
}

struct EchoHandler;

#[async_trait::async_trait]
impl MessageHandler for EchoHandler {
    async fn handle_message(&self, message: &rustycord::Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content.starts_with("!echo ") {
            let response = message.content.strip_prefix("!echo ").unwrap_or("");
            Ok(Some(response.to_string()))
        } else {
            Ok(None)
        }
    }
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustycord = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Getting Started

Ready to build your first Discord bot? Check out our [Getting Started Guide](getting-started/installation.md) to learn how to set up your development environment and create your first bot.

## Examples

- [Basic Bot](examples/basic-bot.md) - Simple echo bot
- [Message Handler](examples/message-handler.md) - Custom message handling
- [Embeds](examples/embeds.md) - Rich embed messages
- [Logging](examples/logging.md) - Comprehensive logging setup

## Documentation

- [User Guide](user-guide/bot-basics.md) - Learn the fundamentals
- [API Reference](api/client.md) - Complete API documentation
- [Examples](examples/basic-bot.md) - Working code examples

## Community

- [GitHub Issues](https://github.com/iamdhakrey/rustycord/issues) - Bug reports and feature requests
- [Discussions](https://github.com/iamdhakrey/rustycord/discussions) - Community support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

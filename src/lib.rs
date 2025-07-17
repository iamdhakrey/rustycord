//! # rustycord - Discord Bot Library for Rust
//!
//! ⚠️ **DEVELOPMENT WARNING**: This library is in heavy development and is **NOT ready for production use**.
//!
//! - APIs change frequently without notice
//! - Features are incomplete and experimental  
//! - Breaking changes occur regularly
//! - Use only for development and testing
//! - **Do not use for production bots**
//!
//! Wait for the stable 1.0 release before using in production.
//!
//! ## Current Status
//!
//! ✅ Basic bot functionality working  
//! ✅ Message handling implemented  
//! ✅ Prefix command system functional  
//! ❌ Slash commands not implemented  
//! ❌ Voice support not available  
//! ❌ API stability not guaranteed  
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use rustycord::{bot::BotBase, gateway::intents};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let token = std::env::var("DISCORD_TOKEN")?;
//!     let intents = intents::GUILDS | intents::GUILD_MESSAGES | intents::MESSAGE_CONTENT;
//!     
//!     let mut bot = BotBase::new(Some(intents)).await;
//!     bot.login(token).await;
//!     bot.connect(Some(intents), Some(true)).await;
//!     
//!     Ok(())
//! }
//! ```

// extern crate log;

pub mod bot;
pub mod client;
pub mod http;
pub mod logger;
pub mod response;
mod user;
mod utils;
// mod wsocks;
pub mod application;
pub mod embeds;
pub mod event_handler;
pub mod gateway;
pub mod handlers;
pub mod message;
pub mod models;
pub mod prefix;

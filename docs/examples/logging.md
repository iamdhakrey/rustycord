# Logging Example

Comprehensive guide to logging in rustycord applications.

## Overview

RustyCord provides a sophisticated logging system built on top of Rust's `log` crate with `fern` for configuration. The logging system supports multiple output targets, log levels, and custom formatting.

## Basic Logging Setup

```rust
use rustycord::{Bot, logger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic logging setup
    logger::setup_logger("info".to_string())?;
    
    log::info!("🚀 Starting RustyCord bot...");
    
    let token = std::env::var("DISCORD_TOKEN")?;
    let mut bot = Bot::new(None).await;
    
    log::info!("🔑 Logging in to Discord...");
    let user_response = bot.login(token).await;
    log::info!("✅ Logged in as: {}", user_response.username);
    
    log::info!("🎯 Starting bot event loop...");
    bot.start().await?;
    
    Ok(())
}
```

## Log Levels

RustyCord supports five log levels:

```rust
use log::{trace, debug, info, warn, error};

// TRACE: Very detailed information, typically only of interest when diagnosing problems
trace!("🔍 Processing message ID: {}", message.id);

// DEBUG: Detailed information, typically only of interest when diagnosing problems
debug!("🐛 Message handler executed: {}", handler_name);

// INFO: General information about program execution
info!("📢 Bot connected to Discord Gateway");

// WARN: Something unexpected happened, but the program can continue
warn!("⚠️ Rate limit approaching for endpoint: {}", endpoint);

// ERROR: Serious problems that might cause the program to abort
error!("❌ Failed to send message: {}", error);
```

## Advanced Logging Configuration

```rust
use rustycord::logger;
use log::LevelFilter;
use fern::Dispatch;
use chrono::Local;

fn setup_advanced_logging() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .level_for("rustycord::gateway", LevelFilter::Debug)
        .level_for("rustycord::http", LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("bot.log")?)
        .apply()?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_advanced_logging()?;
    
    log::info!("🚀 Bot starting with advanced logging...");
    
    // Your bot code here...
    
    Ok(())
}
```

## Structured Logging

```rust
use serde_json::json;

// Structured logging for better analytics
fn log_command_execution(user_id: &str, command: &str, execution_time: u64, success: bool) {
    let log_data = json!({
        "event": "command_execution",
        "user_id": user_id,
        "command": command,
        "execution_time_ms": execution_time,
        "success": success,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    if success {
        info!("Command executed: {}", log_data);
    } else {
        warn!("Command failed: {}", log_data);
    }
}

// Usage in message handler
#[async_trait]
impl MessageHandler for LoggingHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = std::time::Instant::now();
        
        if message.content.starts_with("!test") {
            debug!("🧪 Processing test command from user: {}", message.author.username);
            
            let result = self.process_test_command(message).await;
            let execution_time = start_time.elapsed().as_millis() as u64;
            
            log_command_execution(
                &message.author.id,
                "test",
                execution_time,
                result.is_ok()
            );
            
            result
        } else {
            Ok(None)
        }
    }
}
```

## File Logging with Rotation

```rust
use fern::Dispatch;
use log::LevelFilter;
use chrono::Local;
use std::fs;

fn setup_file_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create logs directory if it doesn't exist
    fs::create_dir_all("logs")?;
    
    let log_filename = format!("logs/bot-{}.log", Local::now().format("%Y-%m-%d"));
    
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(
            Dispatch::new()
                .filter(|metadata| metadata.target().starts_with("rustycord"))
                .chain(fern::log_file(&log_filename)?)
        )
        .chain(
            Dispatch::new()
                .level(LevelFilter::Warn)
                .chain(std::io::stderr())
        )
        .apply()?;
    
    info!("📁 Logging to file: {}", log_filename);
    Ok(())
}
```

## Performance Logging

```rust
use std::time::Instant;

struct PerformanceLogger;

impl PerformanceLogger {
    fn log_function_duration<F, R>(func_name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        if duration.as_millis() > 100 {
            warn!("⏱️ Slow operation: {} took {}ms", func_name, duration.as_millis());
        } else {
            debug!("⚡ Fast operation: {} took {}ms", func_name, duration.as_millis());
        }
        
        result
    }
    
    async fn log_async_duration<F, Fut, R>(func_name: &str, f: F) -> R
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        
        info!("🚀 Async operation: {} completed in {}ms", func_name, duration.as_millis());
        result
    }
}

// Usage example
impl MessageHandler for PerformanceMonitoringHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content == "!performance" {
            let result = PerformanceLogger::log_async_duration("database_query", || async {
                // Simulate database query
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                "Query completed"
            }).await;
            
            Ok(Some(format!("Performance test: {}", result)))
        } else {
            Ok(None)
        }
    }
}
```

## Error Logging with Context

```rust
use anyhow::{Context, Result};

async fn process_user_command(message: &Message) -> Result<String> {
    debug!("🔄 Processing command from user: {}", message.author.username);
    
    let user_data = fetch_user_data(&message.author.id)
        .await
        .with_context(|| format!("Failed to fetch data for user {}", message.author.id))?;
    
    let processed_data = process_data(user_data)
        .with_context(|| "Failed to process user data")?;
    
    info!("✅ Successfully processed command for user: {}", message.author.username);
    Ok(processed_data)
}

async fn fetch_user_data(user_id: &str) -> Result<String> {
    // Simulate potential failure
    if user_id == "error_user" {
        error!("❌ Simulated error for user: {}", user_id);
        anyhow::bail!("User not found in database");
    }
    
    debug!("📊 Fetched data for user: {}", user_id);
    Ok(format!("data_for_{}", user_id))
}

fn process_data(data: String) -> Result<String> {
    if data.contains("invalid") {
        error!("❌ Invalid data format: {}", data);
        anyhow::bail!("Invalid data format");
    }
    
    debug!("🔄 Processing data: {}", data);
    Ok(format!("processed_{}", data))
}
```

## Logging Best Practices

### 1. Use Appropriate Log Levels

```rust
// ✅ Good: Appropriate log levels
debug!("🔍 Validating message format");
info!("📢 User {} joined the server", username);
warn!("⚠️ Rate limit reached, slowing down requests");
error!("❌ Failed to connect to database: {}", error);

// ❌ Bad: Wrong log levels
error!("User sent a message"); // This should be debug or trace
info!("Database connection failed"); // This should be error
```

### 2. Include Context

```rust
// ✅ Good: Includes relevant context
info!("📨 Message received in #{} from {}: {}", 
      channel_name, username, message_preview);
warn!("⚠️ Rate limit hit for endpoint {} (remaining: {})", 
      endpoint, remaining_requests);

// ❌ Bad: No context
info!("Message received");
warn!("Rate limit hit");
```

### 3. Use Emojis for Visual Scanning

```rust
trace!("🔍 Detailed debug info");
debug!("🐛 Debug information");
info!("📢 General information");
warn!("⚠️ Warning message");
error!("❌ Error occurred");
```

### 4. Log Structured Data

```rust
// For important events, log structured data
info!(
    "user_action"; 
    "user_id" => %user.id,
    "action" => "message_sent",
    "channel_id" => %channel.id,
    "message_length" => message.content.len(),
    "timestamp" => %chrono::Utc::now()
);
```

## Production Logging Setup

```rust
use fern::Dispatch;
use log::LevelFilter;

pub fn setup_production_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();
    
    let level = match log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                chrono::Utc::now().to_rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        // Reduce noise from dependencies
        .level_for("hyper", LevelFilter::Warn)
        .level_for("reqwest", LevelFilter::Warn)
        .level_for("tungstenite", LevelFilter::Warn)
        // File output for persistent logs
        .chain(fern::log_file("bot.log")?)
        // Console output for immediate feedback
        .chain(std::io::stdout())
        .apply()?;
    
    info!("🚀 Logging initialized at level: {}", log_level);
    Ok(())
}
```

## Monitoring and Alerts

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct LoggingMetrics {
    error_count: AtomicU64,
    warn_count: AtomicU64,
    last_error_time: AtomicU64,
}

impl LoggingMetrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            error_count: AtomicU64::new(0),
            warn_count: AtomicU64::new(0),
            last_error_time: AtomicU64::new(0),
        })
    }
    
    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
        self.last_error_time.store(
            chrono::Utc::now().timestamp() as u64,
            Ordering::Relaxed
        );
        
        let error_count = self.error_count.load(Ordering::Relaxed);
        if error_count % 10 == 0 {
            warn!("🚨 High error rate detected: {} errors", error_count);
        }
    }
    
    pub fn record_warning(&self) {
        self.warn_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_stats(&self) -> (u64, u64, u64) {
        (
            self.error_count.load(Ordering::Relaxed),
            self.warn_count.load(Ordering::Relaxed),
            self.last_error_time.load(Ordering::Relaxed),
        )
    }
}
```

## Testing Logging

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;

    #[test]
    fn test_logging_setup() {
        // Test that logging can be initialized
        assert!(logger::setup_logger("debug".to_string()).is_ok());
        
        // Test logging at different levels
        log::debug!("Test debug message");
        log::info!("Test info message");
        log::warn!("Test warning message");
        log::error!("Test error message");
    }
    
    #[test]
    fn test_log_levels() {
        let levels = vec!["trace", "debug", "info", "warn", "error"];
        
        for level in levels {
            assert!(logger::setup_logger(level.to_string()).is_ok());
        }
    }
}
```

## Related Documentation

- [Basic Bot Example](basic-bot.md) - Simple logging usage
- [Message Handler Example](message-handler.md) - Logging in handlers
- [Error Handling Guide](../user-guide/error-handling.md) - Error logging patterns

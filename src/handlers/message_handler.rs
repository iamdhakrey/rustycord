use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::Client;
use crate::message::ChannelMessage;

/// Result type for message handler operations
pub type MessageHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// Trait for handling different types of messages
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handle a message create event
    async fn on_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult;

    /// Handle a message update event
    async fn on_message_update(
        &self,
        _message: &ChannelMessage,
        _client: &Client,
    ) -> MessageHandlerResult {
        // Default implementation - can be overridden
        Ok(())
    }

    /// Handle a message delete event
    async fn on_message_delete(
        &self,
        _message_id: &str,
        _channel_id: &str,
        _client: &Client,
    ) -> MessageHandlerResult {
        // Default implementation - can be overridden
        Ok(())
    }
}

/// A simple echo message handler that responds to messages
pub struct EchoMessageHandler;

#[async_trait]
impl MessageHandler for EchoMessageHandler {
    async fn on_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        // Don't respond to bot messages
        if message.author.bot.unwrap_or(false) {
            log::trace!("🤖 Ignoring bot message from: {}", message.author.name);
            return Ok(());
        }

        log::debug!(
            "🔄 Echoing message from {}: {}",
            message.author.name,
            message.content
        );
        // Echo the message back
        let response = format!("Echo: {}", message.content);
        client
            .send_message(&message.channel_id, &response, None)
            .await?;

        Ok(())
    }
}

/// A ping-pong message handler
pub struct PingPongHandler;

#[async_trait]
impl MessageHandler for PingPongHandler {
    async fn on_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        // Don't respond to bot messages
        if message.author.bot.unwrap_or(false) {
            log::trace!("🤖 Ignoring bot message from: {}", message.author.name);
            return Ok(());
        }

        if message.content.to_lowercase().contains("ping") {
            log::debug!(
                "🏓 Ping detected from {}, responding with pong",
                message.author.name
            );
            client
                .send_message(&message.channel_id, "Pong! 🏓", None)
                .await?;
        }

        Ok(())
    }
}

/// Command-based message handler with prefix support
pub struct CommandHandler {
    prefix: String,
    commands: HashMap<
        String,
        Box<dyn Fn(&ChannelMessage, &Client) -> MessageHandlerResult + Send + Sync>,
    >,
}

impl CommandHandler {
    pub fn new(prefix: String) -> Self {
        log::info!("🎯 Creating new CommandHandler with prefix: '{}'", prefix);
        Self {
            prefix,
            commands: HashMap::new(),
        }
    }

    pub fn add_command<F>(&mut self, name: String, handler: F)
    where
        F: Fn(&ChannelMessage, &Client) -> MessageHandlerResult + Send + Sync + 'static,
    {
        log::debug!("📝 Adding command '{}{}' to handler", self.prefix, name);
        self.commands.insert(name, Box::new(handler));
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    pub fn get_commands(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }
}

/// Enhanced prefix-based listener registry
pub struct PrefixListenerRegistry {
    /// Map of prefix -> command handlers
    prefix_handlers: Arc<RwLock<HashMap<String, Arc<CommandHandler>>>>,
    /// Default message handlers (non-prefix based)
    default_handlers: Arc<RwLock<Vec<Box<dyn MessageHandler>>>>,
}

impl PrefixListenerRegistry {
    pub fn new() -> Self {
        log::info!("🎯 Creating new PrefixListenerRegistry");
        Self {
            prefix_handlers: Arc::new(RwLock::new(HashMap::new())),
            default_handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a command handler with a specific prefix
    pub async fn register_prefix_handler(&self, prefix: String, handler: CommandHandler) {
        let mut handlers = self.prefix_handlers.write().await;
        log::info!(
            "✅ Registering prefix handler: '{}' with {} commands",
            prefix,
            handler.get_commands().len()
        );
        log::debug!(
            "🎯 Available commands for '{}': {:?}",
            prefix,
            handler.get_commands()
        );
        handlers.insert(prefix, Arc::new(handler));
    }

    /// Add a default message handler (non-prefix based)
    pub async fn add_default_handler<H>(&self, handler: H)
    where
        H: MessageHandler + 'static,
    {
        let mut handlers = self.default_handlers.write().await;
        let handler_name = std::any::type_name::<H>();
        log::debug!("📝 Adding default message handler: {}", handler_name);
        handlers.push(Box::new(handler));
        log::info!(
            "✅ Default handler registered: {} (Total: {})",
            handler_name,
            handlers.len()
        );
    }

    /// Process a message through all registered handlers
    pub async fn process_message(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        // Don't process bot messages
        if message.author.bot.unwrap_or(false) {
            log::trace!("🤖 Ignoring bot message from: {}", message.author.name);
            return Ok(());
        }

        log::debug!(
            "🔄 Processing message: '{}' from {}",
            message.content,
            message.author.name
        );

        // First try prefix-based handlers
        let prefix_handlers = self.prefix_handlers.read().await;
        let mut handled = false;

        for (prefix, handler) in prefix_handlers.iter() {
            if message.content.starts_with(prefix) {
                log::debug!("🎯 Matched prefix '{}' for message", prefix);
                if let Err(e) = handler.on_message_create(message, client).await {
                    log::error!("❌ Error in prefix handler '{}': {:?}", prefix, e);
                } else {
                    handled = true;
                    log::trace!("✅ Successfully processed with prefix '{}'", prefix);
                }
                break; // Only match first prefix
            }
        }

        // If no prefix handler matched, try default handlers
        if !handled {
            let default_handlers = self.default_handlers.read().await;
            log::debug!(
                "🔄 No prefix match, trying {} default handler(s)",
                default_handlers.len()
            );

            for (index, handler) in default_handlers.iter().enumerate() {
                log::trace!("🎯 Executing default handler {} for message", index + 1);
                if let Err(e) = handler.on_message_create(message, client).await {
                    log::error!("❌ Error in default handler {}: {:?}", index + 1, e);
                }
            }
        }

        Ok(())
    }

    /// Get list of all registered prefixes
    pub async fn get_registered_prefixes(&self) -> Vec<String> {
        let handlers = self.prefix_handlers.read().await;
        handlers.keys().cloned().collect()
    }

    /// Get commands for a specific prefix
    pub async fn get_prefix_commands(&self, prefix: &str) -> Option<Vec<String>> {
        let handlers = self.prefix_handlers.read().await;
        handlers.get(prefix).map(|h| h.get_commands())
    }
}

#[async_trait]
impl MessageHandler for CommandHandler {
    async fn on_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        // Don't respond to bot messages
        if message.author.bot.unwrap_or(false) {
            log::trace!("🤖 Ignoring bot message from: {}", message.author.name);
            return Ok(());
        }

        if message.content.starts_with(&self.prefix) {
            let command_part = &message.content[self.prefix.len()..];
            let parts: Vec<&str> = command_part.split_whitespace().collect();

            if let Some(command_name) = parts.first() {
                log::debug!(
                    "🎯 Attempting to execute command: '{}{}'",
                    self.prefix,
                    command_name
                );
                if let Some(handler) = self.commands.get(*command_name) {
                    log::trace!(
                        "✅ Found handler for command: '{}{}'",
                        self.prefix,
                        command_name
                    );
                    handler(message, client)?;
                } else {
                    log::debug!(
                        "❓ No handler found for command: '{}{}'",
                        self.prefix,
                        command_name
                    );
                }
            }
        }

        Ok(())
    }
}

/// Message handler registry that manages multiple handlers
pub struct MessageHandlerRegistry {
    handlers: Arc<RwLock<Vec<Box<dyn MessageHandler>>>>,
    prefix_registry: Arc<PrefixListenerRegistry>,
}

impl MessageHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
            prefix_registry: Arc::new(PrefixListenerRegistry::new()),
        }
    }

    /// Get access to the prefix listener registry
    pub fn get_prefix_registry(&self) -> Arc<PrefixListenerRegistry> {
        self.prefix_registry.clone()
    }

    pub async fn add_handler<H>(&self, handler: H)
    where
        H: MessageHandler + 'static,
    {
        let mut handlers = self.handlers.write().await;
        let handler_name = std::any::type_name::<H>();
        log::debug!("📝 Adding message handler: {}", handler_name);
        handlers.push(Box::new(handler));
        log::info!(
            "✅ Message handler registered: {} (Total: {})",
            handler_name,
            handlers.len()
        );
    }

    pub async fn handle_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        // First try the new prefix-based system
        if let Err(e) = self.prefix_registry.process_message(message, client).await {
            log::error!("❌ Error in prefix registry: {:?}", e);
        }

        // Also process with legacy handlers for backward compatibility
        let handlers = self.handlers.read().await;
        log::debug!(
            "🔄 Processing MESSAGE_CREATE with {} legacy handler(s)",
            handlers.len()
        );

        for (index, handler) in handlers.iter().enumerate() {
            log::trace!(
                "🎯 Executing legacy handler {} for message: {}",
                index + 1,
                message.content
            );
            if let Err(e) = handler.on_message_create(message, client).await {
                log::error!("❌ Error in legacy message handler {}: {:?}", index + 1, e);
            }
        }

        Ok(())
    }

    pub async fn handle_message_update(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> MessageHandlerResult {
        let handlers = self.handlers.read().await;
        log::debug!(
            "🔄 Processing MESSAGE_UPDATE with {} handler(s)",
            handlers.len()
        );

        for (index, handler) in handlers.iter().enumerate() {
            log::trace!(
                "🎯 Executing update handler {} for message: {}",
                index + 1,
                message.content
            );
            if let Err(e) = handler.on_message_update(message, client).await {
                log::error!("❌ Error in message update handler {}: {:?}", index + 1, e);
            }
        }

        Ok(())
    }

    pub async fn handle_message_delete(
        &self,
        message_id: &str,
        channel_id: &str,
        client: &Client,
    ) -> MessageHandlerResult {
        let handlers = self.handlers.read().await;
        log::debug!(
            "🔄 Processing MESSAGE_DELETE with {} handler(s)",
            handlers.len()
        );
        log::debug!(
            "🗑️ Message deleted - ID: {}, Channel: {}",
            message_id,
            channel_id
        );

        for (index, handler) in handlers.iter().enumerate() {
            log::trace!(
                "🎯 Executing delete handler {} for message ID: {}",
                index + 1,
                message_id
            );
            if let Err(e) = handler
                .on_message_delete(message_id, channel_id, client)
                .await
            {
                log::error!("❌ Error in message delete handler {}: {:?}", index + 1, e);
            }
        }

        Ok(())
    }
}

impl Default for MessageHandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

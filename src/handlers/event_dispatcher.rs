use std::sync::Arc;

use crate::message::ChannelMessage;
use crate::client::Client;
use crate::gateway::response::{GatewayReceiveEventName, ReceiveEvent};
use crate::handlers::message_handler::MessageHandlerRegistry;

/// Event dispatcher that routes gateway events to appropriate handlers
pub struct EventDispatcher {
    message_handlers: Arc<MessageHandlerRegistry>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            message_handlers: Arc::new(MessageHandlerRegistry::new()),
        }
    }
    
    pub fn get_message_handlers(&self) -> Arc<MessageHandlerRegistry> {
        self.message_handlers.clone()
    }
    
    /// Dispatch a gateway event to the appropriate handler
    pub async fn dispatch_event(&self, event: &ReceiveEvent, client: &Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match event.t {
            GatewayReceiveEventName::MESSAGE_CREATE => {
                if let Some(data) = &event.d {
                    match serde_json::from_value::<ChannelMessage>(data.clone()) {
                        Ok(message) => {
                            log::info!("📨 Message received: {} from {}", message.content, message.author.name);
                            self.message_handlers.handle_message_create(&message, client).await?;
                        }
                        Err(e) => {
                            log::error!("Failed to parse MESSAGE_CREATE event: {:?}", e);
                        }
                    }
                }
            }
            GatewayReceiveEventName::MESSAGE_UPDATE => {
                if let Some(data) = &event.d {
                    match serde_json::from_value::<ChannelMessage>(data.clone()) {
                        Ok(message) => {
                            log::info!("📝 Message updated: {} from {}", message.content, message.author.name);
                            self.message_handlers.handle_message_update(&message, client).await?;
                        }
                        Err(e) => {
                            log::error!("Failed to parse MESSAGE_UPDATE event: {:?}", e);
                        }
                    }
                }
            }
            GatewayReceiveEventName::MESSAGE_DELETE => {
                if let Some(data) = &event.d {
                    if let (Some(message_id), Some(channel_id)) = (
                        data.get("id").and_then(|v| v.as_str()),
                        data.get("channel_id").and_then(|v| v.as_str()),
                    ) {
                        log::info!("🗑️ Message deleted: {} in channel {}", message_id, channel_id);
                        self.message_handlers.handle_message_delete(message_id, channel_id, client).await?;
                    }
                }
            }
            GatewayReceiveEventName::READY => {
                log::info!("🚀 Bot is ready!");
            }
            GatewayReceiveEventName::GUILD_CREATE => {
                if let Some(data) = &event.d {
                    if let Some(guild_name) = data.get("name").and_then(|v| v.as_str()) {
                        log::info!("🏰 Joined guild: {}", guild_name);
                    }
                }
            }
            _ => {
                log::debug!("Unhandled event: {:?}", event.t);
            }
        }
        
        Ok(())
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

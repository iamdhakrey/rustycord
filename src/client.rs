// client.rs

use std::sync::Arc;
use tokio::{signal, sync::Mutex};

use crate::{
    gateway::{gateway::PresenceUpdate, shard_manager::ShardManager},
    http::{HTTPClient, MessageResponse},
    response::UserResponse,
    handlers::event_dispatcher::EventDispatcher,
    embeds::Embed,
};

#[derive(Clone)]
pub struct Client {
    pub token: Option<String>,
    pub http: HTTPClient,
    pub shard_manager: Option<Arc<Mutex<ShardManager>>>, // Add ShardManager
    pub event_dispatcher: Arc<EventDispatcher>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            token: None,
            http: HTTPClient::new(),
            shard_manager: None,
            event_dispatcher: Arc::new(EventDispatcher::new()),
        }
    }

    pub async fn login(&mut self, token: String) -> UserResponse {
        self.token = Some(token.clone());
        let res = self.http.login(token).await;
        res
    }

    pub async fn get_gateway(&self) -> String {
        self.http.get_gateway().await
    }

    /// Send a message to a channel
    pub async fn send_message(&self, channel_id: &str, content: &str, embeds: Option<Vec<Embed>>) -> Result<MessageResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.http.send_message(channel_id, content, embeds).await
    }

    /// Send a simple text message to a channel
    pub async fn send_text_message(&self, channel_id: &str, content: &str) -> Result<MessageResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.send_message(channel_id, content, None).await
    }

    /// Send a message with embeds to a channel
    pub async fn send_embed_message(&self, channel_id: &str, embeds: Vec<Embed>) -> Result<MessageResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.send_message(channel_id, "", Some(embeds)).await
    }

    /// Get the event dispatcher for registering message handlers
    pub fn get_event_dispatcher(&self) -> Arc<EventDispatcher> {
        self.event_dispatcher.clone()
    }

    /// Initialize the Shard Manager with the desired number of shards
    pub async fn initialize_shards(&mut self, total_shards: usize) {
        let token = self.token.clone().expect("Token must be set before initializing shards");
        let shard_manager = ShardManager::new(token, total_shards, self.clone());
        self.shard_manager = Some(Arc::new(Mutex::new(shard_manager)));
    }

    /// Start all shards managed by the ShardManager
    pub async fn start_shards(&self, intents: Option<i32>, presense: Option<PresenceUpdate>) {
        if let Some(manager) = &self.shard_manager {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager.lock().await.start(intents, presense).await;
            });
        }
    }

    pub async fn wait_for_shutdown(&self) {
        // Listen for CTRL+C
        signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
        log::info!("Shutdown signal received, shutting down shards...");

        if let Some(_manager) = &self.shard_manager {
            // Implement shard shutdown logic here
            // For example, send a shutdown signal to all shard tasks
            // This can be done using channels or other synchronization primitives
        }

        log::info!("All shards have been shut down.");
    }

      pub async fn ws_connect(
        &mut self,
        intents: Option<i32>,
        #[allow(unused_variables)] reconnect: Option<bool>,
        // compress: bool,
        // large_threshold: Option<i32>,
        // shard: usize,
        presence: Option<PresenceUpdate>,
    ) {
        self.initialize_shards(2).await;
        self.start_shards(intents,presence).await;
    }
}



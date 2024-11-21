// gateway/shard_manager.rs

use futures::future::join_all;

use crate::client::Client;
use crate::gateway::gateway::Manager;

use super::gateway::PresenceUpdate;

pub struct ShardManager {
    pub total_shards: usize,
    pub token: String,
    pub client: Client,
}

impl ShardManager {
    pub fn new(token: String, total_shards: usize, client: Client) -> Self {
        Self {
            total_shards,
            token,
            client,
        }
    }

    pub async fn start(&self, intents: Option<i32>, presense: Option<PresenceUpdate>) {
        let mut shard_tasks = Vec::new();

        for shard_id in 0..self.total_shards {
            let token = self.token.clone();
            let intents: i32 = match intents {
                Some(int) => int,
                None => {
                    log::warn!("No intents provided using default");
                    1
                }
            };
            let intents = intents;
            let compress = false;
            let large_threshold = Some(50);
            let shard = Some(vec![shard_id as i32, self.total_shards as i32]);
            let presence = presense.clone(); // Initialize with your desired presence if any

            // Clone necessary data for the shard
            // let client_clone = self.client.clone();

            // Spawn each shard as a separate task
            let shard_task = async move {
                let mut shard_manager = Manager::new(token.clone(), intents, shard_id, self.total_shards).await;
                shard_manager.send_identify(
                    token,
                    intents,
                    compress,
                    large_threshold,
                    shard,
                    presence,
                ).await;

                loop {
                    // Handle heartbeats
                    shard_manager.handle_heartbeat().await;

                    // Handle incoming events
                    shard_manager.handle_event().await;

                    // Sleep for a short duration to prevent tight looping
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            };

            shard_tasks.push(shard_task);
        }

        // Await all shard tasks
        join_all(shard_tasks).await;
    }
}

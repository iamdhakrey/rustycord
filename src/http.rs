use std::process::exit;

use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::embeds::Embed;
use crate::response::UserResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayResponse {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: String,
    pub channel_id: String,
    pub content: String,
    pub timestamp: String,
    pub author: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub tts: Option<bool>,
    pub nonce: Option<String>,
    pub allowed_mentions: Option<AllowedMentions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedMentions {
    pub parse: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
    pub users: Option<Vec<String>>,
    pub replied_user: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct HTTPClient {
    client: Client,
    // ws: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    token: String,
}

impl HTTPClient {
    const USER_AGENT: &'static str = "rustycord dev";
    const API_URL: &'static str = "https://discord.com/api";

    pub fn new() -> Self {
        HTTPClient {
            client: Client::new(),
            // ws: None,
            token: String::new(),
        }
    }

    pub async fn login(&mut self, token: String) -> UserResponse {
        log::info!("🔑 Attempting to login with provided token");
        log::debug!(
            "🌐 Making request to Discord API: {}/users/@me",
            Self::API_URL
        );
        self.token = token.clone();
        let res = self
            .client
            .get(format!("{}/users/@me", Self::API_URL))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", Self::USER_AGENT)
            .send()
            .await;

        match res {
            Ok(res) => {
                log::debug!("📡 Received response with status: {}", res.status());
                if res.status().is_success() {
                    let user_response = res.json::<UserResponse>().await.unwrap();
                    log::info!(
                        "✅ Successfully authenticated as: {}",
                        user_response.username
                    );
                    user_response
                } else {
                    log::error!("❌ Authentication failed - Invalid token");
                    panic!("🔴 Invalid token");
                }
            }
            Err(err) => {
                log::error!("❌ Network error during authentication: {:?}", err);
                exit(2)
            }
        }
    }

    pub async fn get_gateway(&self) -> String {
        log::debug!("🚪 Fetching gateway URL from Discord API");
        let res = self
            .client
            .get(format!("{}/gateway/bot", Self::API_URL))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", Self::USER_AGENT)
            .send()
            .await;

        match res {
            Ok(res) => {
                log::debug!("📡 Gateway response status: {}", res.status());
                if res.status().is_success() {
                    let gateway_response = res.json::<GatewayResponse>().await.unwrap();
                    let gateway_url = format!("{}/?encoding=json&v=10", gateway_response.url);
                    log::info!("🚪 Gateway URL obtained: {}", gateway_url);
                    gateway_url
                } else {
                    log::error!("❌ Failed to get gateway URL - Invalid token");
                    panic!("🔴 Invalid token");
                }
            }
            Err(err) => {
                log::error!("❌ Network error while fetching gateway: {:?}", err);
                panic!("Error: {:?}", err);
            }
        }
    }
    pub async fn logout(&self) -> bool {
        true
        // self.client
        // .post(format!("{}/auth/logout", Self::API_URL))
        // .header("Authorization",  format!("Bot {}", self.token))
    }

    pub async fn send_message(
        &self,
        channel_id: &str,
        content: &str,
        embeds: Option<Vec<Embed>>,
    ) -> Result<MessageResponse, Box<dyn std::error::Error + Send + Sync>> {
        let endpoint = format!("channels/{}/messages", channel_id);
        let url = format!("{}/{}", Self::API_URL, endpoint);

        log::debug!("📤 Preparing message payload for channel: {}", channel_id);
        let mut payload = json!({
            "content": content,
            "tts": false,
        });

        if let Some(embeds) = embeds {
            log::debug!("📎 Adding {} embed(s) to message", embeds.len());
            payload["embeds"] = serde_json::to_value(embeds)?;
        }

        log::info!("📤 Sending message to channel {}: {}", channel_id, content);
        log::trace!("🔍 Full payload: {}", payload);

        let res = self
            .client
            .post(&url)
            .header("User-Agent", Self::USER_AGENT)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bot {}", self.token))
            .json(&payload)
            .send()
            .await?;

        log::debug!("📡 Message API response status: {}", res.status());

        if res.status().is_success() {
            let response = res.json::<MessageResponse>().await?;
            log::info!("✅ Message sent successfully: {}", response.id);
            Ok(response)
        } else {
            let status = res.status();
            let error_text = res.text().await?;
            log::error!(
                "❌ Failed to send message (status: {}): {}",
                status,
                error_text
            );
            Err(format!("Failed to send message: {}", error_text).into())
        }
    }
}

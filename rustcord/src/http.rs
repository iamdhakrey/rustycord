use std::process::exit;

use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::response::UserResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayResponse {
    url: String,
}

#[derive(Debug, Clone)]
pub struct HTTPClient {
    client: Client,
    // ws: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    token: String,
}

impl HTTPClient {
    const USER_AGENT: &'static str = "Rustcord dev";
    const API_URL: &'static str = "https://discord.com/api";

    pub fn new() -> Self {
        HTTPClient {
            client: Client::new(),
            // ws: None,
            token: String::new(),
        }
    }

    pub async fn login(&mut self, token: String) -> UserResponse {
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
                if res.status().is_success() {
                    res.json::<UserResponse>().await.unwrap()
                } else {
                    panic!("🔴 Invalid token");
                }
            }
            Err(err) => {
                log::error!("Error: {:?}", err);
                exit(2)
            }
        }
    }

    pub async fn get_gateway(&self) -> String {
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
                if res.status().is_success() {
                    // println!("{:?}", res.text().await);
                    // "s".to_string()
                    let _tr = res.json::<GatewayResponse>().await.unwrap();
                    format!("{}/?encoding=json&v=10", _tr.url)
                    // _tr.url
                } else {
                    panic!("🔴 Invalid token");
                }
            }
            Err(err) => {
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

    pub async fn post(&self, endpoint: String, data: String, token:String) -> bool {
        let url = format!("{}/{}", Self::API_URL, endpoint);
        log::info!("🔵 POST: {}", url);
        log::info!("🔵 Data: {}", data);
        let payload = json!({
            "content": "Hello, World!",
            "tts": false,
            "embeds": [{
                "title": "Hello, Embed!",
                "description": "This is an embedded message."
            }]
        });
        let res = self
            .client
            .post(format!("{}/{}", Self::API_URL, endpoint))
            .header("User-Agent", Self::USER_AGENT)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bot {}", token))
            .json(&payload)
            .send().await;
        match res {
            Ok(res) => {
                if res.status().is_success() {
                    log::info!("🔵 Success: {:?}", res.text().await);
                    true
                } else {
                    log::error!("🔴 Error: {:?}", res.text().await);
                    false
                }
            }
            Err(err) => {
                log::error!("Error: {:?}", err);
                false
            }
            
        }
    }
    pub async fn message_create(&self) -> bool {
        true
    }
    // pub async fn ws_connect(&self) -> bool {
    //     true
    //     // get the gateway
    //     // let gateway = self.get_gateway().await;
    //     // log::debug!("{:?}", gateway);

    //     // match tungstenite::connect(gateway) {
    //     //     Ok((mut ws, _)) => {
    //     //         log::info!("🔌 Connected to gateway");
    //     //         loop {
    //     //             let msg = ws.read();
    //     //             match msg {
    //     //                 Ok(msg) => {
    //     //                     log::debug!("Received: {:?}", msg.into_text());
    //     //                 }
    //     //                 Err(err) => {
    //     //                     log::error!("🔴 Error connecting to gateway: {:?}", err);
    //     //                     break;
    //     //                 }
    //     //             }
    //     //         }
    //     //         true
    //     //     }
    //     //     Err(err) => {
    //     //         log::error!("🔴 Error connecting to gateway: {:?}", err);
    //     //         false
    //     //     }
    //     // }
    // }
}

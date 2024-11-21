use std::{process::exit, time::Instant};

const GATEWAY_VERSION: i8 = 10;
const GATEWAY_BASE_URL: &str = "wss://gateway.discord.gg/";
/// The WebSocket connection to the Discord Gateway.
/// This will be used to interact with the Discord Gateway.
use crate::{
    gateway::response::{
        DiscordOpCode, DiscordReceiveEvent, GatewayReceiveEventName, ReceiveEvent,
    },
    http::HTTPClient,
    message::ChannelMessage,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::net::TcpStream;

use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Deserialize, Serialize)]
pub enum WebSocketReceiveData {}

/// Receive Message from the gateway
/// https://discord.com/developers/docs/topics/gateway-events#receive-events
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiveMessageOpcode {
    /// The event type.
    op: i8,
}

/// Identifies the client to the gateway.
/// https://discord.com/developers/docs/topics/gateway-events#identify-identify-connection-properties
#[derive(Debug, Serialize)]
pub struct IdentifyProperties {
    /// Your operating system.
    os: String,

    /// Your library name.
    browser: String,

    /// Your library name.
    device: String,
}

/// https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-types
#[derive(Debug, Serialize, Clone)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

/// The activity object that the user is doing.
/// https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-structure
///
/// Note Bot users are only able to set name, state, type, and url.
#[derive(Debug, Serialize, Clone)]
pub struct Activity {
    /// The activity's name.
    pub name: String,

    /// Activity type.
    // #[serde(rename = "type")]
    pub r#type: u8,
    // Stream url, is validated when type is 1.
    pub url: Option<String>,

    // User's current party status
    pub state: Option<String>,
}

/// Sent by the client to indicate a presence or status update.
/// https://discord.com/developers/docs/topics/gateway-events#update-presence
#[derive(Debug, Serialize, Clone)]
pub struct PresenceUpdate {
    /// Unix time (in milliseconds) of when the client went idle, or null if the client is not idle.
    pub since: u128,

    /// Array of activity objects.
    pub activities: Vec<Activity>,

    /// The user's new status.
    pub status: String,

    /// Whether or not the client is afk.
    pub afk: bool,
}

/// The activity object that the user is doing.
#[derive(Debug, Serialize)]
#[serde(untagged)]

pub enum WebSocketMessageData {
    /// the heartbeat interval that the client should heartbeat to.
    /// https://discord.com/developers/docs/topics/gateway-events#heartbeat/
    Heartbeat(u64),

    /// Used to trigger the initial handshake with the gateway.
    /// https://discord.com/developers/docs/topics/gateway-events#identify-identify-structure
    Identify {
        /// authentication token
        token: String,
        /// Gateway Intents you wish to receive.
        intents: i32,

        /// The properties for the client.
        properties: IdentifyProperties,

        /// Whether this connection supports compression of packets.
        compress: bool,

        /// Value between 50 and 250, total number of members
        /// where the gateway will stop sending offline members
        /// in the guild member list.
        large_threshold: i32,

        /// Used for Guild Sharding.
        /// A list of two integers, [shard_id, num_shards].
        shard: Option<Vec<i32>>,

        /// Presence structure for initial presence information.
        presence: Option<PresenceUpdate>,
    },

    /// Sent by the client to indicate a presence or status update.
    /// https://discord.com/developers/docs/topics/gateway-events#update-presence
    PresenceUpdate(PresenceUpdate),
}
//  "properties": {"$os": "linux", "$browser": "rustcord", "$device": "rustcord"};

#[derive(Debug, Serialize)]
pub struct WebSocketMessage {
    op: u8,
    d: WebSocketMessageData,
}

pub struct RustCordWebSocketResponse {}

pub struct DiscordWebSocket(WebSocketStream<MaybeTlsStream<TcpStream>>);

/// The WebSocket connection to the Discord Gateway.
/// This is the main struct that will be used to interact with the Discord Gateway.
/// It will be used to send and receive messages from the gateway.
/// It will also be used to handle the connection to the gateway.
/// The connection will be maintained by sending heartbeats to the gateway every 5 seconds.
/// The connection will be authenticated by sending an identify message to the gateway.
/// The connection will be reconnected if the gateway disconnects.
/// The connection will be resumed if the gateway disconnects.
///
/// # Methods
///
/// * `connect` - Connect to the Discord Gateway using the provided client
/// * `recv` - Receive message from the gateway in json format
/// * `send_json` - Send message to the gateway in json format
/// * `send_heartbeat` - Send heartbeat to the gateway every 5 seconds
/// * `send_identify` - Send identify message to the gateway to authenticate the client
/// * `presence_update` - Send presence update to the gateway
///
#[allow(dead_code)]
impl DiscordWebSocket {
    /// All gateway events in Discord are tagged with an opcode that denotes the payload type.
    /// Your connection to our gateway may also sometimes close. When it does, you will
    /// receive a close code that tells you what happened.
    /// https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
    ///

    /// An event was dispatched.
    /// Client Action: Receive only
    const DISPATCH: u8 = 0;

    /// Fired periodically by the client to keep the connection alive.
    /// Client Action: Send/Receive
    const HEARTBEAT: u8 = 1;

    /// Starts a new session during the initial handshake.
    /// Client Action: Send only
    const IDENTIFY: u8 = 2;

    /// Update the client's presence.
    /// Client Action: Send only
    const PRESENCE_UPDATE: u8 = 3;

    /// Used to join/leave or move between voice channels.
    /// Client Action: Send only
    const VOICE_STATE_UPDATE: u8 = 4;

    /// Resume a previous session that was disconnected.
    /// Client Action: Send only
    const RESUME: u8 = 6;

    /// You should attempt to reconnect and resume immediately.
    /// Client Action: Receive only
    const RECONNECT: u8 = 7;

    /// Request information about offline guild members in a large guild.
    /// Client Action: Send only
    const REQUEST_GUILD_MEMBERS: u8 = 8;

    /// The session has been invalidated. You should reconnect and identify/resume accordingly.
    /// Client Action: Receive only
    const INVALID_SESSION: u8 = 9;

    /// Sent immediately after connecting, contains the `heartbeat_interval` to use.
    /// Client Action: Receive only
    const HELLO: u8 = 10;

    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    /// Client Action: Receive only
    const HEARTBEAT_ACK: u8 = 11;

    /// Connect to the Discord Gateway using the provided client
    ///
    /// # Arguments
    ///
    /// * `client` - the client to connect to the gateway
    pub(crate) async fn connect(shard_id: usize) -> Self {
        let gateway_url = format!("{}?v={}&encoding=json", GATEWAY_BASE_URL, GATEWAY_VERSION);
        // let (ws_stream, _) = connect_async(gateway_url).await.unwrap();
        match connect_async(gateway_url).await {
            Ok((ws_stream, _)) => {
                log::info!("🔌 {} -> Connected to The Discord", shard_id);
                Self(ws_stream)
            }
            Err(err) => {
                log::error!("Shard {} -> Got Error Message {}", shard_id, err);
                exit(2)
            }
        }
        // Self(ws_stream)
    }

    /// receive message from the gateway in json format
    pub async fn recv(&mut self) -> Result<(Option<ReceiveEvent>, bool), String> {
        let message = self.0.next().await.unwrap().unwrap();
        let message = match message {
            Message::Text(text) => text,
            _ => "".to_string(),
        };
        if message.is_empty() {
            return Err("🔴 -> Error receiving message".to_string());
        }
        let _opcode = from_str::<ReceiveMessageOpcode>(message.as_str()).unwrap();

        let r_event = from_str::<DiscordReceiveEvent>(message.as_str()).unwrap();

        // change discord event to rustcord event
        let r_event = ReceiveEvent {
            t: r_event.t,
            s: r_event.s,
            op: DiscordOpCode::from(_opcode.op),
            d: r_event.d,
        };

        Ok((Some(r_event), true))
    }

    /// send message to the gateway in json format
    ///
    /// # Arguments
    ///
    /// * `message` - the message to send to the gateway
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustcord::gateway::gateway::WebSocketMessage;
    /// use rustcord::gateway::gateway::WebSocketMessageData;
    /// use rustcord::gateway::gateway::DiscordWebSocket;
    /// use rustcord::gateway::gateway::PresenceUpdate;
    ///
    /// let mut ws = DiscordWebSocket::connect().await.unwrap();
    /// let message = WebSocketMessage {
    ///    op: 1,
    ///   d: WebSocketMessageData::Heartbeat(251),
    /// };
    /// ws.send_json(message).await;
    /// ```
    pub async fn send_json(&mut self, message: WebSocketMessage) -> bool {
        let message = Message::Text(serde_json::to_string(&message).unwrap());
        match self.0.send(message).await {
            Ok(_) => true,

            Err(err) => {
                log::error!("Error sending message {}", err);
                false
            }
        }
    }

    /// send heartbeat to the gateway every 5 seconds
    pub async fn send_heartbeat(&mut self) -> bool {
        let heartbeat = WebSocketMessage {
            op: Self::HEARTBEAT,
            d: WebSocketMessageData::Heartbeat(251),
        };
        // log::debug!("💓 -> Sending heartbeat");
        self.send_json(heartbeat).await
    }

    /// send identify message to the gateway to authenticate the client
    ///
    /// # Arguments
    ///
    /// * `token` - the token of the bot
    /// * `compress` - whether the connection supports compression of packets
    /// * `large_threshold` - Value between 50 and 250, total number of members
    ///              where the gateway will stop sending offline members in
    ///             the guild member list (default 50)
    /// * `shard` - used for Guild Sharding. A list of two integers, [shard_id, num_shards].
    /// * `presence` - presence structure for initial presence information
    pub async fn send_identify(
        &mut self,
        token: String,
        intents: Option<i32>,
        compress: bool,
        large_threshold: Option<i32>,
        shard: Option<Vec<i32>>,
        presence: Option<PresenceUpdate>,
    ) {
        let int: i32 = match intents {
            Some(int) => int,
            None => {
                log::warn!("No intents provided using default");
                1
            }
        };
        let pre = presence.unwrap_or_else(|| PresenceUpdate {
            since: 0,
            activities: vec![Activity {
                name: "with Rust".to_string(),
                r#type: 0,
                url: Some("https://iamdhakrey.dev".to_string()),
                state: Some("Enjoying Rust".to_string()),
            }],
            status: "online".to_string(),
            afk: false,
        });
        let shr: Vec<i32> = shard.unwrap_or_else(|| vec![0, 1]);
        let identify = WebSocketMessage {
            op: DiscordWebSocket::IDENTIFY,
            d: WebSocketMessageData::Identify {
                token: token.to_string(),
                intents: int,
                properties: IdentifyProperties {
                    os: "linux".to_string(),
                    browser: "rustcord".to_string(),
                    device: "rustcord".to_string(),
                },
                compress: compress,
                large_threshold: large_threshold.unwrap_or(50),
                shard: Some(shr),
                presence: Some(pre),
            },
        };
        log::info!("🔑 -> Sending Identification Message");
        self.send_json(identify).await;
    }

    /// send presence update to the gateway
    ///
    /// # Arguments
    ///
    /// * `presence` - presence structure for initial presence information
    async fn presence_update(&mut self, presence: PresenceUpdate) {
        let activities: Vec<Activity> = presence.activities.clone();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let presence = WebSocketMessage {
            op: Self::PRESENCE_UPDATE,
            d: WebSocketMessageData::PresenceUpdate(PresenceUpdate {
                since: now,
                activities: activities,
                status: "online".to_string(),
                afk: false,
            }),
        };

        self.send_json(presence).await;
    }

    async fn resume(&mut self) {
        let resume = WebSocketMessage {
            op: Self::RESUME,
            d: WebSocketMessageData::Heartbeat(251),
        };
        self.send_json(resume).await;
    }
}

pub struct Manager {
    pub ws: DiscordWebSocket,
    last_heartbeat: Option<Instant>,
    last_heartbeat_ack: Option<Instant>,
    heartbeat_interval: Option<i8>,

    pub token: String,
    pub intents: i32,
    shard_id: usize,
    pub total_shards: usize,
}

impl Manager {
    pub async fn new(token: String, intents: i32, shard_id: usize, total_shards: usize) -> Self {
        let ws = DiscordWebSocket::connect(shard_id).await;
        let last_heartbeat = None;
        let last_heartbeat_ack = None;
        let heartbeat_interval = 0;
        Self {
            ws,
            last_heartbeat,
            last_heartbeat_ack,
            heartbeat_interval: Some(heartbeat_interval),
            token,
            intents,
            shard_id,
            total_shards,
        }
    }

    /// return false if heartbeat is not required to send to the gateway
    ///
    /// True - where heartbeat is required to send to the gateway
    /// True - where heartbeat interval is not set or last heartbeat ack is more than 15 seconds ago
    /// True - where last heartbeat is less than 5 seconds ago
    ///
    /// False - where last heartbeat is more than 5 seconds ago
    /// False - where heartbeat interval is set and last heartbeat ack is less than 15 seconds ago
    ///
    pub async fn is_required_heartbeat(&mut self) -> bool {
        if self.heartbeat_interval.is_none() {
            log::debug!("Need to send heartbeat");
            return true;
        }
        if self.last_heartbeat_ack.is_none() {
            log::debug!("Need to send heartbeat");
            return true;
        }
        if self.last_heartbeat.is_none() {
            log::debug!("Need to send heartbeat");
            return true;
        }
        if self.last_heartbeat.unwrap().elapsed().as_secs() < 5 {
            log::debug!("NO Need to send heartbeat");
            return false;
        }
        if self.last_heartbeat_ack.unwrap().elapsed().as_secs() > 15 {
            log::debug!("Need to send heartbeat");
            return true;
        }
        if self.last_heartbeat.unwrap().elapsed().as_secs() > 5 {
            log::debug!("Need to send heartbeat");
            return true;
        }
        log::debug!("Need to send heartbeat");
        true
    }

    pub async fn handle_heartbeat(&mut self) {
        if self.is_required_heartbeat().await {
            self.ws.send_heartbeat().await;
            self.last_heartbeat = Some(Instant::now());
        }
    }

    pub async fn handle_event(&mut self) {
        match self.ws.recv().await {
            Ok((Some(event), _)) => {
                log::debug!(
                    "Shard {} received event: {:?} op: {:?}",
                    self.shard_id,
                    event.t,
                    event.op
                );
                let event_clone = event.clone();
                if DiscordOpCode::from(event_clone.op) == DiscordOpCode::HeartbeatAck {
                    self.last_heartbeat_ack = Some(Instant::now())
                } else {
                    self.dispatch(event_clone).await
                }
            }
            Ok((None, _)) => {
                log::warn!("Shard {} received None event", self.shard_id);
            }
            Err(err) => {
                log::error!("Shard {} error receiving event: {:?}", self.shard_id, err);
            }
        }
    }
    /// Dispatch events to handlers
    async fn dispatch(&self, event: ReceiveEvent) {
        self.event_handler(event).await;
    }

    async fn event_handler(&self, event: ReceiveEvent) {
        log::debug!("Shard {} - Event {:?}", self.shard_id, event.t);
        if event.t == GatewayReceiveEventName::READY {
            self.ready().await;
        } else if event.t == GatewayReceiveEventName::MESSAGE_CREATE {
            self.message_create(event).await
        }
    }

    /// Handle READY event
    async fn ready(&self) {
        log::info!("🟢 Shard {} is ready", self.shard_id);
    }

    /// Handle Message Create Event
    async fn message_create(&self, event: ReceiveEvent) {
        if let Some(data) = event.d.clone() {
            log::debug!("Shard {} - Message Data: {:?}", self.shard_id, data);
            let message_result =
                serde_json::from_str::<ChannelMessage>(&serde_json::to_string(&data).unwrap());
            // println!("{:?}", message_result);
            match message_result {
                Ok(message) => {
                    log::info!("Shard {} - 📩 Message: {:?}", self.shard_id, message);
                    self.send_message(message).await
                }
                Err(err) => log::error!("Shard {} - Error parsing message: {}", self.shard_id, err),
            }
        }
    }

    async fn send_message(&self, message: ChannelMessage) {
        log::info!("Shard {} - Sending Message: {:?}", self.shard_id, message);
        let endpoint = format!("channels/{}/messages", message.channel_id);
        let client = HTTPClient::new();
        let new_message = r#"
        {
            "content": "Hello, World!",
            "tts": false,
            "embeds": [{
              "title": "Hello, Embed!",
              "description": "This is an embedded message."
            }]
        }
        "#;
        client
            .post(
                endpoint,
                serde_json::to_string(&new_message).unwrap(),
                self.token.clone(),
            )
            .await;
        // match res {
        //     Ok(res) => {
        //         if res.status().is_success() {
        //             log::info!("Shard {} - Message Sent", self.shard_id);
        //         } else {
        //             log::error!("Shard {} - Error sending message", self.shard_id);
        //         }
        //     }
        //     Err(err) => {
        //         log::error!("Shard {} - Error sending message: {:?}", self.shard_id, err);
        //     }
        // }
    }
    /// Send identify message with shard information
    pub async fn send_identify(
        &mut self,
        token: String,
        intents: i32,
        compress: bool,
        large_threshold: Option<i32>,
        shard: Option<Vec<i32>>,
        presence: Option<PresenceUpdate>,
    ) {
        let identify = WebSocketMessage {
            op: DiscordWebSocket::IDENTIFY,
            d: WebSocketMessageData::Identify {
                token: token.to_string(),
                intents,
                properties: IdentifyProperties {
                    os: "linux".to_string(),
                    browser: "rustcord".to_string(),
                    device: "rustcord".to_string(),
                },
                compress,
                large_threshold: large_threshold.unwrap_or(50),
                shard,
                presence,
            },
        };
        log::info!(
            "🔑 Shard {} - Sending Identification Message",
            self.shard_id
        );
        self.ws.send_json(identify).await;
    }
}

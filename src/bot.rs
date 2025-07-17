use serde::Serialize;

use crate::{
    client::Client, gateway::gateway::PresenceUpdate, logger::setup_logger, response::UserResponse,
};

#[derive(Debug, Serialize)]
pub struct ClientBuilderWS {
    pub intents: Option<i32>,
    pub reconnect: Option<bool>,
    pub shard: usize,
    pub presence: Option<PresenceUpdate>,
}
pub struct BotBase {
    pub intents: Option<i32>,
    pub client: Option<Client>,
    pub presence: Option<PresenceUpdate>,
}

impl BotBase {
    pub async fn new(intents: Option<i32>) -> Self {
        log::debug!("🤖 Creating new BotBase instance");
        if let Some(intents) = intents {
            log::debug!("🎯 Bot intents configured: {}", intents);
        } else {
            log::debug!("🎯 Bot intents not specified, will use defaults");
        }
        BotBase {
            intents: intents,
            client: None,
            presence: None,
        }
    }

    /// login the bot
    ///
    /// # Examples
    ///
    /// ```
    /// use rustycord::bot::BotBase;
    ///
    /// let mut bot_base = ;
    /// assert_eq!(bot_base.login(token), );
    /// assert_eq!(bot_base, );
    /// ```
    pub async fn login(&mut self, token: String) -> UserResponse {
        log::info!("🔑 Initializing bot login...");
        let mut _client = Client::new();
        let res = _client.login(token).await;
        log::info!("🔒 Logged in as: {:?}", res.username);
        self.client = Some(_client);
        res
    }

    /// start the bot
    ///
    /// # Arguments
    ///
    /// * `token` - the token of the bot
    /// * `reconnect` - if the bot should reconnect
    ///
    pub async fn start(&mut self, token: String, reconnect: Option<bool>) -> () {
        log::info!("🚀 Starting bot...");
        self.login(token).await;
        log::info!("📡 Establishing WebSocket connection...");
        self.connect(self.intents, reconnect).await;
    }

    pub async fn stop(&self) -> bool {
        true
    }

    pub async fn connect(&mut self, intents: Option<i32>, reconnect: Option<bool>) {
        log::debug!("🌐 Connecting to Discord gateway...");
        self.client
            .as_mut()
            .unwrap()
            .ws_connect(intents, reconnect, None)
            .await;
        log::info!("✅ Successfully connected to Discord gateway");
    }

    pub async fn set_presence(&mut self, presence: PresenceUpdate) {
        self.presence = Some(presence);
    }

    // pub async fn set_shard(&mut self, shard: usize) {
    //     let ws = ClientBuilderWS {
    //         intents: self.intents,
    //         reconnect: Some(true),
    //         shard: shard,
    //         presence: self.presence,
    //     };
    // }
    pub async fn run(&mut self, token: String, log_level: Option<String>) -> bool {
        let level: String = log_level.unwrap_or_else(|| "info".to_string());

        log::info!("⚙️ Initializing logger with level: {}", level);
        let _ = setup_logger(level);
        log::info!("🤖 rustycord bot starting up...");
        self.start(token, Some(true)).await;
        true
    }
}

pub struct Bot {}

impl Bot {
    pub async fn builder(intents: Option<i32>) -> BotBase {
        let bot = BotBase::new(intents).await;
        bot
    }
}

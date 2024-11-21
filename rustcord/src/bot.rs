use log::info;
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
    /// use rustcord::bot::BotBase;
    ///
    /// let mut bot_base = ;
    /// assert_eq!(bot_base.login(token), );
    /// assert_eq!(bot_base, );
    /// ```
    pub async fn login(&mut self, token: String) -> UserResponse {
        let mut _client = Client::new();
        let res = _client.login(token).await;
        info!("🔒 Logged in as: {:?}", res.username);
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
        self.login(token).await;

        self.connect(self.intents, reconnect).await;
    }

    pub async fn stop(&self) -> bool {
        true
    }

    pub async fn connect(&mut self, intents: Option<i32>, reconnect: Option<bool>) {
        // client = Client::new()
        self.client
            .as_mut()
            .unwrap()
            .ws_connect(intents, reconnect, None)
            .await;
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

        // get log level
        let _ = setup_logger(level);
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

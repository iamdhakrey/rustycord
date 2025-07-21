use serde::{Deserialize, Serialize};

use crate::{application::Application, embeds::Embed, models::user::User};

/// Represent a role tags
///
/// More: <https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct RoleTags {
    /// the id of the bot this role belongs to
    pub bot_id: Option<String>,

    /// the id of the integration this role belongs to
    pub integration_id: Option<String>,

    /// whether this is the guild's premium subscriber role
    pub premium_subscriber: Option<bool>,
}

/// Represent a role in a guild
///
/// More: <https://discord.com/developers/docs/topics/permissions#role-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    /// The id of the role
    pub id: String,

    /// The name of the role
    pub name: String,

    /// The color of the role
    pub color: i32,

    /// if this role is pinned in the user listing
    pub hoist: bool,

    /// role's icon hash
    pub icon: Option<String>,

    /// role's unicode emoji
    pub unicode_emoji: Option<String>,

    /// position of this role
    pub position: i32,

    /// permission bit set
    pub permissions: i32,

    /// whether this role is managed by an integration
    pub managed: bool,

    /// whether this role is mentionable
    pub mentionable: bool,

    /// the tags this role has
    pub tags: Option<RoleTags>,

    /// role flags combined as a bitfield
    #[serde(default)]
    pub flags: i32,
}

/// Represent an attachment in a message
///
/// More: <https://discord.com/developers/docs/resources/channel#attachment-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct Attachments {
    /// the id of the attachment
    pub id: String,

    /// the name of the file attached
    pub filename: String,

    /// title
    pub title: Option<String>,

    /// description
    pub description: Option<String>,

    /// the media type of the attachment
    pub content_type: String,

    /// the size of the file in bytes
    pub size: i32,

    /// the source url of the file
    pub url: String,

    /// the proxied url of the file
    pub proxy_url: String,

    /// the height of the file (if it is an image)
    pub height: Option<i32>,

    /// the width of the file (if it is an image)
    pub width: Option<i32>,

    /// whether the attachment is ephemeral
    pub ephemeral: Option<bool>,

    /// the duration of the audio file
    pub duration: Option<i32>,

    /// base64 encoded bytearray representing a sampled waveform of the audio file
    pub waveform: Option<Vec<i32>>,

    /// flag the attachment as being the spoiler of the content
    pub spoiler: Option<bool>,
}

/// Represent a emoji reaction to a message
#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    /// Total number of times this emoji has been used to react (including super reacts)
    pub count: i32,

    /// reaction count details object
    pub count_details: Vec<ReactionCountDetails>,

    /// whether the current user reacted using this emoji
    pub me: bool,

    /// whether this emoji is a super react
    pub me_react: bool,

    /// emoji information
    pub emoji: Emoji,

    /// Hex color used for super reacts
    pub burst_color: Option<String>,
}

/// Reaction count details object
#[derive(Serialize, Deserialize, Debug)]
pub struct ReactionCountDetails {
    /// count of super reacts
    pub burst: i32,

    /// count of normal reacts
    pub normal: i32,
}

/// Emoji information
/// More: <https://discord.com/developers/docs/resources/emoji#emoji-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    /// emoji id
    pub id: Option<String>,

    /// emoji name
    pub name: Option<String>,

    /// roles this emoji is whitelisted to
    pub roles: Option<Vec<Role>>,

    /// user that created this emoji
    pub user: Option<User>,

    /// whether this emoji must be wrapped in colons
    pub require_colons: Option<bool>,

    /// whether this emoji is managed
    pub managed: Option<bool>,

    /// whether this emoji is animated
    pub animated: Option<bool>,

    /// whether this emoji can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>,
}

/// Represent a message activity
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageActivity {
    /// type of message activity
    ///
    /// Message Activity Types
    ///     
    /// | Type | Value |
    /// |------|-------|
    /// | JOIN | 1 |
    /// | SPECTATE | 2 |
    /// | LISTEN | 3 |
    /// | JOIN_REQUEST | 5 |
    pub r#type: i32,

    /// party_id from a Rich Presence event
    pub party_id: Option<String>,
}

/// Represent a message sent in a channel
///
///
///
/// - Note: Field specific to the `MESSAGE_CREATE` and `MESSAGE_UPDATE` events are listed in the Gateway Documentation
/// - Note: An app will receive empty values in the `content`, `embeds`, `attachement` and `component` fields while `poll`
/// will be ommited if they have not configured the `MESSAGE_CONTENT` intent
///
/// More: <https://discord.com/developers/docs/resources/message#message-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelMessage {
    /// The id of the message
    pub id: String,

    /// The id of the channel the message was sent in
    pub channel_id: String,

    /// The author of the message( not guaranteed to be a valid user)
    ///
    /// The author object follows the structure of the user object,
    /// but is only a valid user in the case where the message is
    /// generated by a user or bot user if the message is generated by a webhook
    /// the author object corresponds to the webhook's id, username, and avatar
    /// you can tell if a message is generated by a webhook by checking
    /// for the `webhook_id` on the message object
    pub author: User,

    /// contents of the message
    ///
    /// An app will receive empty values in the `content`, `embeds`, `attachement` and `component` fields while `poll`
    /// will be ommited if they have not configured the `MESSAGE_CONTENT` intent
    pub content: String,

    /// when this message was sent
    pub timestamp: String,

    /// when this message was edited
    pub edited_timestamp: Option<String>,

    /// whether this was a TTS message
    /// TTS messages are read aloud using text-to-speech when received by clients
    /// that support TTS
    ///
    pub tts: bool,

    /// whether this message mentions everyone
    pub mention_everyone: bool,

    /// users specifically mentioned in the message
    pub mentions: Vec<User>,

    /// roles specifically mentioned in this message
    pub mention_roles: Vec<Role>,

    /// attachments sent with the message
    pub attachments: Vec<Attachments>,

    /// any embedded content
    pub embeds: Vec<Embed>,

    /// reactions to the message
    pub reactions: Option<Vec<Reaction>>,

    /// used for validating a message was sent
    pub nonce: Option<String>,

    /// whether this message is pinned
    pub pinned: bool,

    /// if the message is generated by a webhook, this is the webhook's id
    pub webhook_id: Option<String>,

    /// type of message
    ///
    /// TODO: <https://discord.com/developers/docs/resources/message#message-object-message-types>
    pub r#type: i32,

    /// sent with Rich Presence-related chat embeds
    pub activity: Option<MessageActivity>,

    /// application of the message
    pub application: Option<Application>,

    /// application id
    #[serde(default)]
    pub application_id: String,

    /// message flags combinded as a bitfield
    #[serde(default)]
    pub flags: usize,
}

/// Represent a message sent in a channel
///
/// More: <https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure>
///
/// - Note: Field specific to the `MESSAGE_CREATE` and `MESSAGE_UPDATE` events are listed in the Gateway Documentation
/// - Note: An app will receive empty values in the `content`, `embeds`, `attachement` and `component` fields while `poll`
impl ChannelMessage {
    pub fn new(
        id: String,
        channel_id: String,
        author: User,
        content: String,
        timestamp: String,
        tts: bool,
        mention_everyone: bool,
        mentions: Vec<User>,
        mention_roles: Vec<Role>,
        attachments: Vec<Attachments>,
        embeds: Vec<Embed>,
        reactions: Option<Vec<Reaction>>,
        nonce: Option<String>,
        pinned: bool,
        webhook_id: Option<String>,
        r#type: i32,
        activity: Option<MessageActivity>,
        application: Option<Application>,
        application_id: String,
        flags: usize,
    ) -> Self {
        Self {
            id,
            channel_id,
            author,
            content,
            timestamp,
            edited_timestamp: None,
            tts,
            mention_everyone,
            mentions,
            mention_roles,
            attachments,
            embeds,
            reactions,
            nonce,
            pinned,
            webhook_id,
            r#type,
            activity,
            application,
            application_id,
            flags,
        }
    }

    pub async fn send(&self, message: String) -> () {
        println!("Message sent: {}", message);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum GatewayReceiveEventName {
    /// Defines the Heartbeat interval
    HELLO,

    /// Contains the initial state information
    READY,

    /// Response to Resume
    RESUMED,

    /// Server is going away, client should reconnect to gateway and resume
    RECONNECT,

    /// Failure response to Identify or Resume or invalid active session
    INVALID_SESSION,

    /// Application command permissions was updated
    APPLICATION_COMMAND_PERMISSIONS_UPDATE,

    /// Auto Moderation rule was created
    AUTO_MODERATION_RULE_CREATE,

    /// Auto Moderation rule was updated
    AUTO_MODERATION_RULE_UPDATE,

    /// Auto Moderation rule was deleted
    AUTO_MODERATION_RULE_DELETE,

    /// Auto Moderation rule was triggered and an action was executed
    AUTO_MODERATION_ACTION_EXECUTION,

    /// New guild channel created
    CHANNEL_CREATE,

    /// Channel was updated
    CHANNEL_UPDATE,

    /// Channel was deleted
    CHANNEL_DELETE,

    /// Message was pinned or unpinned
    CHANNEL_PINS_UPDATE,

    /// Thread created, also sent when being added to a private thread
    THREAD_CREATE,

    /// Thread was updated
    THREAD_UPDATE,

    /// Thread was deleted
    THREAD_DELETE,

    /// Sent when gaining access to a channel, contains all active threads in that channel
    THREAD_LIST_SYNC,

    /// Thread member for the current user was updated
    THREAD_MEMBER_UPDATE,

    /// Some user(s) were added to or removed from a thread
    THREAD_MEMBERS_UPDATE,

    /// Entitlement was created
    ENTITLEMENT_CREATE,

    /// Entitlement was updated or renewed
    ENTITLEMENT_UPDATE,

    /// Entitlement was deleted
    ENTITLEMENT_DELETE,

    /// Lazy-load for unavailable guild, guild became available, or user joined a new guild
    GUILD_CREATE,

    /// Guild was updated
    GUILD_UPDATE,

    /// Guild became unavailable, or user left/was removed from a guild
    GUILD_DELETE,

    /// A guild audit log entry was created
    GUILD_AUDIT_LOG_ENTRY_CREATE,

    /// User was banned from a guild
    GUILD_BAN_ADD,

    /// User was unbanned from a guild
    GUILD_BAN_REMOVE,

    /// Guild emojis were updated
    GUILD_EMOJIS_UPDATE,

    /// Guild stickers were updated
    GUILD_STICKERS_UPDATE,

    /// Guild integration was updated
    GUILD_INTEGRATIONS_UPDATE,

    /// New user joined a guild
    GUILD_MEMBER_ADD,

    /// User was removed from a guild
    GUILD_MEMBER_REMOVE,

    /// Guild member was updated
    GUILD_MEMBER_UPDATE,

    /// Response to Request Guild Members
    GUILD_MEMBERS_CHUNK,

    /// Guild role was created
    GUILD_ROLE_CREATE,

    /// Guild role was updated
    GUILD_ROLE_UPDATE,

    /// Guild role was deleted
    GUILD_ROLE_DELETE,

    /// Guild scheduled event was created
    GUILD_SCHEDULED_EVENT_CREATE,

    /// Guild scheduled event was updated
    GUILD_SCHEDULED_EVENT_UPDATE,

    /// Guild scheduled event was deleted
    GUILD_SCHEDULED_EVENT_DELETE,

    /// User subscribed to a guild scheduled event
    GUILD_SCHEDULED_EVENT_USER_ADD,

    /// User unsubscribed from a guild scheduled event
    GUILD_SCHEDULED_EVENT_USER_REMOVE,

    /// Guild integration was created
    INTEGRATION_CREATE,

    /// Guild integration was updated
    INTEGRATION_UPDATE,

    /// Guild integration was deleted
    INTEGRATION_DELETE,

    /// User used an interaction, such as an Application Command
    INTERACTION_CREATE,

    /// Invite to a channel was created
    INVITE_CREATE,

    /// Invite to a channel was deleted
    INVITE_DELETE,

    /// Message was created
    MESSAGE_CREATE,

    /// Message was edited
    MESSAGE_UPDATE,

    /// Message was deleted
    MESSAGE_DELETE,

    /// Multiple messages were deleted at once
    MESSAGE_DELETE_BULK,

    /// User reacted to a message
    MESSAGE_REACTION_ADD,

    /// User removed a reaction from a message
    MESSAGE_REACTION_REMOVE,

    /// All reactions were explicitly removed from a message
    MESSAGE_REACTION_REMOVE_ALL,

    /// All reactions for a given emoji were explicitly removed from a message
    MESSAGE_REACTION_REMOVE_EMOJI,

    /// User was updated
    PRESENCE_UPDATE,

    /// Stage instance was created
    STAGE_INSTANCE_CREATE,

    /// Stage instance was updated
    STAGE_INSTANCE_UPDATE,

    /// Stage instance was deleted or closed
    STAGE_INSTANCE_DELETE,

    /// User started typing in a channel
    TYPING_START,

    /// Properties about the user changed
    USER_UPDATE,

    /// Someone joined, left, or moved a voice channel
    VOICE_STATE_UPDATE,

    /// Guild's voice server was updated
    VOICE_SERVER_UPDATE,

    /// Guild channel webhook was created, update, or deleted
    WEBHOOKS_UPDATE,

    /// User voted on a poll
    MESSAGE_POLL_VOTE_ADD,

    /// User removed a vote on a poll
    MESSAGE_POLL_VOTE_REMOVE,

    /// Unknown event
    /// This is a catch-all for any event that is not yet implemented
    /// in the library.
    #[serde(untagged)]
    Unknown,
}

impl PartialEq for GatewayReceiveEventName {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::HELLO, Self::HELLO) => true,
            (Self::READY, Self::READY) => true,
            (Self::RESUMED, Self::RESUMED) => true,
            (Self::RECONNECT, Self::RECONNECT) => true,
            (Self::INVALID_SESSION, Self::INVALID_SESSION) => true,
            (
                Self::APPLICATION_COMMAND_PERMISSIONS_UPDATE,
                Self::APPLICATION_COMMAND_PERMISSIONS_UPDATE,
            ) => true,
            (Self::AUTO_MODERATION_RULE_CREATE, Self::AUTO_MODERATION_RULE_CREATE) => true,
            (Self::AUTO_MODERATION_RULE_UPDATE, Self::AUTO_MODERATION_RULE_UPDATE) => true,
            (Self::AUTO_MODERATION_RULE_DELETE, Self::AUTO_MODERATION_RULE_DELETE) => true,
            (Self::AUTO_MODERATION_ACTION_EXECUTION, Self::AUTO_MODERATION_ACTION_EXECUTION) => {
                true
            }
            (Self::CHANNEL_CREATE, Self::CHANNEL_CREATE) => true,
            (Self::CHANNEL_UPDATE, Self::CHANNEL_UPDATE) => true,
            (Self::CHANNEL_DELETE, Self::CHANNEL_DELETE) => true,
            (Self::CHANNEL_PINS_UPDATE, Self::CHANNEL_PINS_UPDATE) => true,
            (Self::THREAD_CREATE, Self::THREAD_CREATE) => true,
            (Self::THREAD_UPDATE, Self::THREAD_UPDATE) => true,
            (Self::THREAD_DELETE, Self::THREAD_DELETE) => true,
            (Self::THREAD_LIST_SYNC, Self::THREAD_LIST_SYNC) => true,
            (Self::THREAD_MEMBER_UPDATE, Self::THREAD_MEMBER_UPDATE) => true,
            (Self::THREAD_MEMBERS_UPDATE, Self::THREAD_MEMBERS_UPDATE) => true,
            (Self::ENTITLEMENT_CREATE, Self::ENTITLEMENT_CREATE) => true,
            (Self::ENTITLEMENT_UPDATE, Self::ENTITLEMENT_UPDATE) => true,
            (Self::ENTITLEMENT_DELETE, Self::ENTITLEMENT_DELETE) => true,
            (Self::GUILD_CREATE, Self::GUILD_CREATE) => true,
            (Self::GUILD_UPDATE, Self::GUILD_UPDATE) => true,
            (Self::GUILD_DELETE, Self::GUILD_DELETE) => true,
            (Self::GUILD_AUDIT_LOG_ENTRY_CREATE, Self::GUILD_AUDIT_LOG_ENTRY_CREATE) => true,
            (Self::GUILD_BAN_ADD, Self::GUILD_BAN_ADD) => true,
            (Self::GUILD_BAN_REMOVE, Self::GUILD_BAN_REMOVE) => true,
            (Self::GUILD_EMOJIS_UPDATE, Self::GUILD_EMOJIS_UPDATE) => true,
            (Self::GUILD_STICKERS_UPDATE, Self::GUILD_STICKERS_UPDATE) => true,
            (Self::GUILD_INTEGRATIONS_UPDATE, Self::GUILD_INTEGRATIONS_UPDATE) => true,
            (Self::GUILD_MEMBER_ADD, Self::GUILD_MEMBER_ADD) => true,
            (Self::GUILD_MEMBER_REMOVE, Self::GUILD_MEMBER_REMOVE) => true,
            (Self::GUILD_MEMBER_UPDATE, Self::GUILD_MEMBER_UPDATE) => true,
            (Self::GUILD_MEMBERS_CHUNK, Self::GUILD_MEMBERS_CHUNK) => true,
            (Self::GUILD_ROLE_CREATE, Self::GUILD_ROLE_CREATE) => true,
            (Self::GUILD_ROLE_UPDATE, Self::GUILD_ROLE_UPDATE) => true,
            (Self::GUILD_ROLE_DELETE, Self::GUILD_ROLE_DELETE) => true,
            (Self::GUILD_SCHEDULED_EVENT_CREATE, Self::GUILD_SCHEDULED_EVENT_CREATE) => true,
            (Self::GUILD_SCHEDULED_EVENT_UPDATE, Self::GUILD_SCHEDULED_EVENT_UPDATE) => true,
            (Self::GUILD_SCHEDULED_EVENT_DELETE, Self::GUILD_SCHEDULED_EVENT_DELETE) => true,
            (Self::GUILD_SCHEDULED_EVENT_USER_ADD, Self::GUILD_SCHEDULED_EVENT_USER_ADD) => true,
            (Self::GUILD_SCHEDULED_EVENT_USER_REMOVE, Self::GUILD_SCHEDULED_EVENT_USER_REMOVE) => {
                true
            }
            (Self::INTEGRATION_CREATE, Self::INTEGRATION_CREATE) => true,
            (Self::INTEGRATION_UPDATE, Self::INTEGRATION_UPDATE) => true,
            (Self::INTEGRATION_DELETE, Self::INTEGRATION_DELETE) => true,
            (Self::INTERACTION_CREATE, Self::INTERACTION_CREATE) => true,
            (Self::INVITE_CREATE, Self::INVITE_CREATE) => true,
            (Self::INVITE_DELETE, Self::INVITE_DELETE) => true,
            (Self::MESSAGE_CREATE, Self::MESSAGE_CREATE) => true,
            (Self::MESSAGE_UPDATE, Self::MESSAGE_UPDATE) => true,
            (Self::MESSAGE_DELETE, Self::MESSAGE_DELETE) => true,
            (Self::MESSAGE_DELETE_BULK, Self::MESSAGE_DELETE_BULK) => true,
            (Self::MESSAGE_REACTION_ADD, Self::MESSAGE_REACTION_ADD) => true,
            (Self::MESSAGE_REACTION_REMOVE, Self::MESSAGE_REACTION_REMOVE) => true,
            (Self::MESSAGE_REACTION_REMOVE_ALL, Self::MESSAGE_REACTION_REMOVE_ALL) => true,
            (Self::MESSAGE_REACTION_REMOVE_EMOJI, Self::MESSAGE_REACTION_REMOVE_EMOJI) => true,
            (Self::PRESENCE_UPDATE, Self::PRESENCE_UPDATE) => true,
            (Self::STAGE_INSTANCE_CREATE, Self::STAGE_INSTANCE_CREATE) => true,
            (Self::STAGE_INSTANCE_UPDATE, Self::STAGE_INSTANCE_UPDATE) => true,
            (Self::STAGE_INSTANCE_DELETE, Self::STAGE_INSTANCE_DELETE) => true,
            (Self::TYPING_START, Self::TYPING_START) => true,
            (Self::USER_UPDATE, Self::USER_UPDATE) => true,
            (Self::VOICE_STATE_UPDATE, Self::VOICE_STATE_UPDATE) => true,
            (Self::VOICE_SERVER_UPDATE, Self::VOICE_SERVER_UPDATE) => true,
            (Self::WEBHOOKS_UPDATE, Self::WEBHOOKS_UPDATE) => true,
            (Self::MESSAGE_POLL_VOTE_ADD, Self::MESSAGE_POLL_VOTE_ADD) => true,
            (Self::MESSAGE_POLL_VOTE_REMOVE, Self::MESSAGE_POLL_VOTE_REMOVE) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy)]
pub enum DiscordOpCode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}

impl PartialEq for DiscordOpCode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Dispatch, Self::Dispatch) => true,
            (Self::Heartbeat, Self::Heartbeat) => true,
            (Self::Identify, Self::Identify) => true,
            (Self::PresenceUpdate, Self::PresenceUpdate) => true,
            (Self::VoiceStateUpdate, Self::VoiceStateUpdate) => true,
            (Self::Resume, Self::Resume) => true,
            (Self::Reconnect, Self::Reconnect) => true,
            (Self::RequestGuildMembers, Self::RequestGuildMembers) => true,
            (Self::InvalidSession, Self::InvalidSession) => true,
            (Self::Hello, Self::Hello) => true,
            (Self::HeartbeatAck, Self::HeartbeatAck) => true,
            _ => false,
        }
    }
}

impl Clone for DiscordOpCode {
    fn clone(&self) -> Self {
        *self
    }
}
// impl for change i8 to DiscordOpCode
impl From<i8> for DiscordOpCode {
    fn from(value: i8) -> Self {
        match value {
            0 => DiscordOpCode::Dispatch,
            1 => DiscordOpCode::Heartbeat,
            2 => DiscordOpCode::Identify,
            3 => DiscordOpCode::PresenceUpdate,
            4 => DiscordOpCode::VoiceStateUpdate,
            6 => DiscordOpCode::Resume,
            7 => DiscordOpCode::Reconnect,
            8 => DiscordOpCode::RequestGuildMembers,
            9 => DiscordOpCode::InvalidSession,
            10 => DiscordOpCode::Hello,
            11 => DiscordOpCode::HeartbeatAck,
            _ => panic!("Invalid OpCode"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordReceiveEvent {
    pub t: GatewayReceiveEventName,
    pub s: Option<i32>,
    pub op: i8,
    pub d: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReceiveEvent {
    pub t: GatewayReceiveEventName,
    pub s: Option<i32>,
    pub op: DiscordOpCode,
    pub d: Option<serde_json::Value>,
}

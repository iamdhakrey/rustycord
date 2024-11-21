use serde::{self, Deserialize, Serialize};

/// User model for the Discord API.
///
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: String,

    #[serde(rename = "username")]
    pub name: String,

    pub discriminator: String,

    pub global_name: Option<String>,

    pub avatar: String,

    pub avatar_description: Option<String>,

    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub system: bool,

    pub banner: Option<String>,

    pub accent_color: Option<u64>,

    #[serde(default)]
    pub mfa_enabled: bool,
    pub locale: Option<String>,

    pub verified: Option<bool>,

    pub email: Option<String>,
    #[serde(default)]
    pub flags: i32,
    #[serde(default)]
    pub premium_type: PremiumType,
    pub public_flags: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum PremiumType {
    #[default]
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3,
}

// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
// pub enum UserPublicFlags {
//     DiscordEmployee = 1 << 0,
//     PartneredServerOwner = 1 << 1,
//     HypeSquadEvents = 1 << 2,
//     BugHunterLevel1 = 1 << 3,
//     HouseBravery = 1 << 6,
//     HouseBrilliance = 1 << 7,
//     HouseBalance = 1 << 8,
//     EarlySupporter = 1 << 9,
//     TeamUser = 1 << 10,
//     System = 1 << 12,
//     BugHunterLevel2 = 1 << 14,
//     VerifiedBot = 1 << 16,
//     EarlyVerifiedBotDeveloper = 1 << 17,
// }

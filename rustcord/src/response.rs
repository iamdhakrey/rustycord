use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: u64,
    pub flags: u64,
    pub bot: bool,
    pub banner: Option<String>,
    pub accent_color: Option<u64>,
    pub global_name: Option<String>,
    pub avatar_description: Option<String>,
    pub banner_color: Option<String>,
    pub clan: Option<String>,
    pub mfa_enabled: bool,
    pub locale: String,
    pub premium_type: u64,
    pub email: Option<String>,
    pub verified: bool,
    pub bio: Option<String>,
}

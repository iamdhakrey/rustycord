// use serde::Deserialize;

// /// User struct that represents a Discord  Base user.
// ///

// struct BaseUser {
//     /// The user's id.
//     pub id: String,

//     /// The user's username, not unique across the platform.
//     pub username: String,

//     /// The user's 4-digit discord-tag.
//     pub discriminator: String,

//     /// The user;s display name, if it is set. For bots, this is the Application Name.
//     pub global_name: Option<String>,

//     /// The user's avatar hash.
//     pub avatar: String,

//     /// whether the user belongs to an OAuth2 application.
//     pub bot: bool,

//     /// whether the user is an Official Discord System user (part of the urgent message system).
//     /// This is only available when using the OAuth2 flow.
//     pub system: bool,

//     /// whether the user has two factor enabled on their account.
//     pub mfa_enabled: bool,

//     /// The User banner hash.
//     pub banner: Option<String>,

//     /// accent color of the user banner.
//     pub accent_color: Option<u64>,

//     /// The user chosen language option.
//     pub locale: String,

//     /// verification status of the user.
//     pub verified: bool,

//     /// The user's email.
//     pub email: Option<String>,

//     /// The flags on a user's account.
//     pub flags: u64,

//     /// The type of Nitro subscription on a user's account.
//     pub premium_type: u64,
//     // The public flags on a user's account.
//     // pub public_flags: u64,

//     // Data for the user;s avatar decoration.
//     // pub avatar_decoration_data: Option<String>,
// }

// #[derive(Deserialize)]
// struct UserPayload {
//     username: String,
//     id: String,
//     discriminator: String,
//     global_name: Option<String>,
//     avatar: Option<String>,
//     banner: Option<String>,
//     accent_color: Option<u64>,
//     bot: Option<bool>,
//     system: Option<bool>,
//     public_flags: Option<u64>,
// }
// impl BaseUser {
//     fn new(data: UserPayload) -> Self {
//         BaseUser {
//             id: data.id,
//             username: data.username,
//             discriminator: data.discriminator,
//             global_name: data.global_name,
//             avatar: data.avatar.unwrap_or_else(|| "".to_string()),
//             bot: data.bot.unwrap_or_else(|| false),
//             system: data.system.unwrap_or_else(|| false),
//             mfa_enabled: false,
//             banner: data.banner,
//             accent_color: data.accent_color,
//             locale: "".to_string(),
//             verified: false,
//             email: None,
//             flags: data.public_flags.unwrap_or_else(|| 0),
//             premium_type: 0,
//         }
//     }

//     pub fn public_flags(&self) -> u64 {
//         self.flags
//     }
// }

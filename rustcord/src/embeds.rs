use serde::{Deserialize, Serialize};

/// Represent an embed in a message
///
/// More: https://discord.com/developers/docs/resources/message#embed-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    /// title of embed
    pub title: Option<String>,

    /// type of embed
    /// (always "rich" for webhook embeds)
    pub r#type: EmbedType,

    /// description of embed
    pub description: Option<String>,

    /// url of embed
    pub url: Option<String>,

    /// timestamp of embed content
    pub timestamp: Option<String>,

    /// color code of the embed
    pub color: Option<i32>,

    /// footer information
    pub footer: Option<EmbedFooter>,

    /// image information
    pub image: Option<EmbedImage>,

    /// thumbnail information
    pub thumbnail: Option<EmbedThumbnail>,

    /// video information
    pub video: Option<EmbedVideo>,

    /// provider information
    pub provider: Option<EmbedProvider>,

    /// author information
    pub author: Option<EmbedAuthor>,

    /// fields information
    pub fields: Option<Vec<EmbedField>>,
}

/// Embed Types
/// Embed types are "loosely defined" and, for the most part,
/// are not used by our clients for rendering. Embed attributes
/// power what is rendered. Embed types should be considered
/// deprecated and might be removed in a future API version.
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum EmbedType {
    /// generic embed rendered from embed attributes
    rich,

    /// image embed
    image,

    /// video embed
    video,

    /// animated gif image embed rendered as a video embed
    gifv,

    /// article embed
    article,

    /// link embed
    link,
}

/// Represent an embed footer
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-footer-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedFooter {
    /// footer text
    pub text: String,

    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,

    /// a proxied url of footer icon
    pub proxy_icon_url: Option<String>,
}

/// Represent an embed image
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-image-structure
#[derive(Serialize, Deserialize, Debug)]

pub struct EmbedImage {
    /// source url of image (only supports http(s) and attachments)
    pub url: Option<String>,

    /// a proxied url of the image
    pub proxy_url: Option<String>,

    /// height of image
    pub height: Option<i32>,

    /// width of image
    pub width: Option<i32>,
}

/// Represent an embed thumbnail
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-thumbnail-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedThumbnail {
    /// source url of thumbnail (only supports http(s) and attachments)
    pub url: Option<String>,

    /// a proxied url of the thumbnail
    pub proxy_url: Option<String>,

    /// height of thumbnail
    pub height: Option<i32>,

    /// width of thumbnail
    pub width: Option<i32>,
}

/// Represent an embed video
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-video-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedVideo {
    /// source url of video
    pub url: Option<String>,

    /// a proxied url of the video
    pub proxy_url: Option<String>,

    /// height of video
    pub height: Option<i32>,

    /// width of video
    pub width: Option<i32>,
}

/// Represent an embed provider
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-provider-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedProvider {
    /// name of provider
    pub name: Option<String>,

    /// url of provider
    pub url: Option<String>,
}

/// Represent an embed author
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-author-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedAuthor {
    /// name of author
    pub name: Option<String>,

    /// url of author
    pub url: Option<String>,

    /// url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,

    /// a proxied url of author icon
    pub proxy_icon_url: Option<String>,
}

/// Represent an embed field
///
/// More: https://discord.com/developers/docs/resources/message#embed-object-embed-field-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedField {
    /// name of the field
    pub name: String,

    /// value of the field
    pub value: String,

    /// whether or not this field should display inline
    pub inline: Option<bool>,
}

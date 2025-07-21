use serde::{Deserialize, Serialize};

/// Represent an embed in a message
///
/// More: <https://discord.com/developers/docs/resources/message#embed-object>
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
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-footer-structure>
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
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-image-structure>
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
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-thumbnail-structure>
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
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-video-structure>
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
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-provider-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedProvider {
    /// name of provider
    pub name: Option<String>,

    /// url of provider
    pub url: Option<String>,
}

/// Represent an embed author
///
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-author-structure>
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
/// More: <https://discord.com/developers/docs/resources/message#embed-object-embed-field-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedField {
    /// name of the field
    pub name: String,

    /// value of the field
    pub value: String,

    /// whether or not this field should display inline
    pub inline: Option<bool>,
}

impl Embed {
    /// Create a new embed with default values
    pub fn new() -> Self {
        Self {
            title: None,
            r#type: EmbedType::rich,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }

    /// Set the title of the embed
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the description of the embed
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set the color of the embed
    pub fn color(mut self, color: i32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the URL of the embed
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Set the timestamp of the embed
    pub fn timestamp(mut self, timestamp: &str) -> Self {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    /// Set the footer of the embed
    pub fn footer(mut self, text: &str, icon_url: Option<&str>) -> Self {
        self.footer = Some(EmbedFooter {
            text: text.to_string(),
            icon_url: icon_url.map(|s| s.to_string()),
            proxy_icon_url: None,
        });
        self
    }

    /// Set the author of the embed
    pub fn author(mut self, name: &str, url: Option<&str>, icon_url: Option<&str>) -> Self {
        self.author = Some(EmbedAuthor {
            name: Some(name.to_string()),
            url: url.map(|s| s.to_string()),
            icon_url: icon_url.map(|s| s.to_string()),
            proxy_icon_url: None,
        });
        self
    }

    /// Add a field to the embed
    pub fn field(mut self, name: &str, value: &str, inline: bool) -> Self {
        let field = EmbedField {
            name: name.to_string(),
            value: value.to_string(),
            inline: Some(inline),
        };

        match self.fields {
            Some(ref mut fields) => fields.push(field),
            None => self.fields = Some(vec![field]),
        }
        self
    }

    /// Set the image of the embed
    pub fn image(mut self, url: &str) -> Self {
        self.image = Some(EmbedImage {
            url: Some(url.to_string()),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Set the thumbnail of the embed
    pub fn thumbnail(mut self, url: &str) -> Self {
        self.thumbnail = Some(EmbedThumbnail {
            url: Some(url.to_string()),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }
}

impl Default for Embed {
    fn default() -> Self {
        Self::new()
    }
}

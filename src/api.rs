use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents a field in an embed, containing a name, value, and an inline flag.
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents the footer of an embed, which can include text and an icon URL.
pub struct Footer {
    pub text: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents an image in an embed, including its URL and optional dimensions.
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents a thumbnail in an embed, including its URL and optional dimensions.
pub struct Thumbnail {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents a video in an embed, including its URL and optional dimensions.
pub struct Video {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents the provider of the content in an embed, including an optional name and URL.
pub struct Provider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[allow(missing_docs)]
/// Represents the author of the content in an embed, including a name, optional URL, and optional icon URL.
pub struct Author {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
/// Represents an [embed](https://discord.com/developers/docs/resources/webhook) message, containing various fields for rich content display.
pub struct Embed {
    /// Title of embed
    pub title: String,
    /// Description of embe
    pub description: Option<String>,
    /// Fields information, max of 25
    pub fields: Vec<Field>,
    /// Timestamp of embed content
    pub timestamp: Option<bool>,
    /// Color code of the embed
    pub color: Option<u32>,
    /// Footer information
    pub footer: Option<Footer>,
    /// Image information
    pub image: Option<Image>,
    /// Thumbnail information
    pub thumbnail: Option<Thumbnail>,
    /// Video information
    pub video: Option<Video>,
    /// Provider information
    pub provider: Option<Provider>,
    /// Author information
    pub author: Option<Author>,
}

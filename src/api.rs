use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Footer {
    pub text: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Thumbnail {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Video {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Provider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
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

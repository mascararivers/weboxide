use serde::Serialize;

#[derive(Serialize)]
pub struct Field {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize)]
pub struct Footer;
#[derive(Serialize)]
pub struct Image;
#[derive(Serialize)]
pub struct Thumbnail;
#[derive(Serialize)]
pub struct Video;
#[derive(Serialize)]
pub struct Provider;
#[derive(Serialize)]
pub struct Author;

#[derive(Serialize, Default)]
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

/// The webhook client interface.
pub struct WebhookClient {
    /// Reqwest client to be used for HTTP requests to Discord API
    pub client: reqwest::Client,
    /// The webhook URL
    pub hook_url: String,
    /// The avatar URL of the webhook user
    pub avatar_url: Option<String>,
    /// The username of the webhook user
    pub username: Option<String>,
    /// The embeds of the webhook
    pub embeds: Vec<Embed>,
}

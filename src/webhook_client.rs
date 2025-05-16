use crate::api::{Author, Embed, Field, Footer, Image, Provider, Thumbnail, Video, WebhookClient};
use crate::error::WeboxideResult;
use serde_json;

struct LooksConfig {
    timestamp: Option<bool>,
    color: Option<u32>,
    footer: Option<Footer>,
    author: Option<Author>,
}

struct MediaConfig {
    image: Option<Image>,
    thumbnail: Option<Thumbnail>,
    video: Option<Video>,
    provider: Option<Provider>,
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

impl WebhookClient {
    pub fn new(
        client: reqwest::Client,
        hook_url: String,
        avatar_url: Option<String>,
        username: Option<String>,
        embeds: Vec<Embed>,
    ) -> WebhookClient {
        WebhookClient {
            client,
            hook_url,
            avatar_url,
            username,
            embeds,
        }
    }

    /// Sends a request to the webhook to send a message
    ///
    /// Fails when request fails, either due to invalid webhook URL or Discord API error
    pub async fn send_message(&self, message: impl Into<String>) -> WeboxideResult<()> {
        let body = serde_json::json!({
            "avatar_url": self.avatar_url,
            "username": self.username,
            "embeds": self.embeds,
            "content": message
        });

        self.client.post(&self.hook_url).json(&body).send().await?;

        Ok(())
    }

    /// Deletes the webhook, removing it from Discord.
    ///
    /// Fails when request fails, either due to invalid webhook URL or Discord API error
    pub async fn delete_hook(&self) -> WeboxideResult<()> {
        self.client.delete(&self.hook_url).send().await?;
        Ok(())
    }

    /// Adds an embed to the webhook client's list of embeds from just the embed title, fields and
    /// description.
    ///
    /// It is advised you run this function before sending the message.
    pub fn add_embed(
        &mut self,
        title: impl Into<String>,
        description: Option<String>,
        fields: Vec<Field>,
    ) {
        self.embeds.push(Embed {
            title: title.into(),
            description,
            fields,
            ..Default::default()
        });
    }

    /// Adds embed an embed to the webhook client's list of embeds from individual embed parts:
    /// - `title`: Embed title information (mandatory)
    /// - `description`: Embed description information
    /// - `fields`: Embed fields information
    /// - `timestamp`: Embed timestamp information
    /// - `color`: Embed color information
    /// - `footer`: Embed footer information
    /// - `image`: Embed image information
    /// - `thumbnail`: Embed thumbnail information
    /// - `video`: Embed video information
    /// - `provider`: Embed provider information
    /// - `author`: Embed author information
    ///
    /// It is advised you run this function before sending the message.
    pub fn add_embed_from_parts(
        &mut self,
        title: impl Into<String>,
        description: Option<String>,
        fields: Vec<Field>,
        looks: LooksConfig,
        media: MediaConfig,
    ) {
        self.embeds.push(Embed {
            title: title.into(),
            description,
            fields,
            color: looks.color,
            timestamp: looks.timestamp,
            footer: looks.footer,
            author: looks.author,
            image: media.image,
            thumbnail: media.thumbnail,
            video: media.video,
            provider: media.provider,
        });
    }
}

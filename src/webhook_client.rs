#![allow(clippy::too_many_arguments)]
use crate::api::{Author, Embed, Field, Footer, Image, Provider, Thumbnail, Video, WebhookClient};
use crate::error::WeboxideResult;
use serde_json;

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
    pub async fn send_message(&self, message: String) -> WeboxideResult<()> {
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
    pub fn add_embed(&mut self, title: String, description: Option<String>, fields: Vec<Field>) {
        self.embeds.push(Embed {
            title,
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
        title: String,
        description: Option<String>,
        fields: Vec<Field>,
        timestamp: Option<bool>,
        color: Option<u32>,
        footer: Option<Footer>,
        image: Option<Image>,
        thumbnail: Option<Thumbnail>,
        video: Option<Video>,
        provider: Option<Provider>,
        author: Option<Author>,
    ) {
        self.embeds.push(Embed {
            title,
            description,
            fields,
            timestamp,
            color,
            footer,
            image,
            thumbnail,
            video,
            provider,
            author,
        });
    }
}

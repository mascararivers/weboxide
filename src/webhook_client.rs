use serde::Serialize;
use serde_json;

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
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,    
    pub timestamp: Option<bool>,
    pub color: Option<u32>,
    pub footer: Option<Footer>,
    pub image: Option<Image>,
    pub thumbnail: Option<Thumbnail>,
    pub video: Option<Video>,
    pub provider: Option<Provider>,
    pub author: Option<Author>
}

pub struct WebhookClient {
    pub client: reqwest::Client,
    pub hook_url: String,
    pub avatar_url: Option<String>,
    pub username: Option<String>,
    pub embeds: Vec<Embed>
}

impl WebhookClient {
    pub fn new(client: reqwest::Client, hook_url: String, avatar_url: Option<String>, username: Option<String>,
    embeds: Vec<Embed>) -> WebhookClient {
        WebhookClient {
            client: client,
            hook_url: hook_url,
            avatar_url: avatar_url,
            username: username,
            embeds: embeds,
        }
    }
    pub fn send_message(&self, message: String) {
        let body = serde_json::json!({
            "avatar_url": self.avatar_url,
            "username": self.username,
            "embeds": self.embeds,
            "content": message 
        });

        let _ = self.client.post(&self.hook_url).body(body.to_string());
    }

    pub fn delete_hook(&self) {
        let _ = self.client.delete(self.hook_url.clone());
    }

    pub fn add_embed(&mut self, title: String, description: Option<String>, fields: Vec<Field>) {
        self.embeds.push(Embed {
            title: title,            
            description: description,
            fields: fields,
            ..Default::default()
        });
    }

    pub fn add_embed_from_parts(&mut self, title: String, description: Option<String>, fields: Vec<Field>,
    timestamp: Option<bool>, color: Option<u32>, footer: Option<Footer>, image: Option<Image>,
    thumbnail: Option<Thumbnail>, video: Option<Video>, provider: Option<Provider>, author: Option<Author>) {
        self.embeds.push(Embed {
            title: title,
            description: description,
            fields: fields,
            timestamp: timestamp,
            color: color,
            footer: footer,
            image: image,
            thumbnail: thumbnail,
            video: video,
            provider: provider,
            author: author,          
        });
    }
}
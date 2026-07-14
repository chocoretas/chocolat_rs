use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use serde::Deserialize;

pub struct Meme;

#[derive(Deserialize)]
struct MemeResponse {
    title: String,
    url: String,
}

#[async_trait::async_trait]
impl Command for Meme {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "meme",
            description: "Obtén un meme aleatorio de Reddit",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let client = reqwest::Client::new();
        let embed = match client.get("https://meme-api.com/gimme").send().await {
            Ok(resp) => {
                if let Ok(meme) = resp.json::<MemeResponse>().await {
                    CreateEmbed::new()
                        .title(&meme.title)
                        .image(&meme.url)
                        .color(0xF39C12)
                } else {
                    CreateEmbed::new().description("No se pudo analizar el meme...").color(0xFF0000)
                }
            }
            Err(_) => {
                CreateEmbed::new().description("Error de red al buscar el meme.").color(0xFF0000)
            }
        };

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Meme
    }
}

use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use serde::Deserialize;

pub struct NekoGirl;

#[derive(Deserialize)]
struct NekoResult {
    url: String,
}

#[derive(Deserialize)]
struct NekoResponse {
    results: Vec<NekoResult>,
}

#[async_trait::async_trait]
impl Command for NekoGirl {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "nekogirl",
            description: "Muestra una linda chica gato anime",
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
        let embed = match client.get("https://nekos.best/api/v2/neko").send().await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<NekoResponse>().await {
                    if let Some(n) = data.results.first() {
                        CreateEmbed::new()
                            .title("🐱 ¡Nyah!")
                            .image(&n.url)
                            .color(0xE91E63)
                    } else {
                        CreateEmbed::new().description("No encontré chicas gato...").color(0xFF0000)
                    }
                } else {
                    CreateEmbed::new().description("Respuesta de neko inválida.").color(0xFF0000)
                }
            }
            Err(_) => {
                CreateEmbed::new().description("Error de red al buscar chica neko.").color(0xFF0000)
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
        command: &NekoGirl
    }
}

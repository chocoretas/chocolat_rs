use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use serde::Deserialize;

pub struct Waifu;

#[derive(Deserialize)]
struct WaifuResult {
    url: String,
}

#[derive(Deserialize)]
struct WaifuResponse {
    results: Vec<WaifuResult>,
}

#[async_trait::async_trait]
impl Command for Waifu {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "waifu",
            description: "Muestra una hermosa waifu aleatoria",
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
        let embed = match client.get("https://nekos.best/api/v2/waifu").send().await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<WaifuResponse>().await {
                    if let Some(w) = data.results.first() {
                        CreateEmbed::new()
                            .title("💖 ¡Aquí tienes a tu Waifu!")
                            .image(&w.url)
                            .color(0x9B59B6)
                    } else {
                        CreateEmbed::new().description("No encontré ninguna waifu...").color(0xFF0000)
                    }
                } else {
                    CreateEmbed::new().description("Respuesta de waifu inválida.").color(0xFF0000)
                }
            }
            Err(_) => {
                CreateEmbed::new().description("Error de red al buscar waifu.").color(0xFF0000)
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
        command: &Waifu
    }
}

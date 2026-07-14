use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use serde::Deserialize;

pub struct Husbando;

#[derive(Deserialize)]
struct HusbandoResult {
    url: String,
}

#[derive(Deserialize)]
struct HusbandoResponse {
    results: Vec<HusbandoResult>,
}

#[async_trait::async_trait]
impl Command for Husbando {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "husbando",
            description: "Muestra un husbando de anime aleatorio",
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
        let embed = match client.get("https://nekos.best/api/v2/husbando").send().await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<HusbandoResponse>().await {
                    if let Some(h) = data.results.first() {
                        CreateEmbed::new()
                            .title("💞 ¡Aquí tienes a tu Husbando!")
                            .image(&h.url)
                            .color(0x34495E)
                    } else {
                        CreateEmbed::new().description("No encontré ningún husbando...").color(0xFF0000)
                    }
                } else {
                    CreateEmbed::new().description("Respuesta de husbando inválida.").color(0xFF0000)
                }
            }
            Err(_) => {
                CreateEmbed::new().description("Error de red al buscar husbando.").color(0xFF0000)
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
        command: &Husbando
    }
}

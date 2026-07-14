use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use serde::Deserialize;

pub struct Trump;

#[derive(Deserialize)]
struct TrumpResponse {
    value: String,
}

#[async_trait::async_trait]
impl Command for Trump {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "trump",
            description: "Obtén una frase célebre (o no tanto) de Donald Trump",
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
        let embed = match client.get("https://api.tronalddump.io/random/quote").send().await {
            Ok(resp) => {
                if let Ok(quote) = resp.json::<TrumpResponse>().await {
                    CreateEmbed::new()
                        .title("🗣️ Donald Trump dice:")
                        .description(format!(""{}"", quote.value))
                        .color(0xF1C40F)
                } else {
                    CreateEmbed::new().description("No se pudo analizar la frase de Trump...").color(0xFF0000)
                }
            }
            Err(_) => {
                CreateEmbed::new().description("Error de red al buscar la frase.").color(0xFF0000)
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
        command: &Trump
    }
}

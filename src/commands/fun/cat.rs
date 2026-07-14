use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use serde::Deserialize;

pub struct Cat;

#[derive(Deserialize)]
struct CatResponse {
    url: String,
}

#[async_trait::async_trait]
impl Command for Cat {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "cat",
            description: "Muestra la foto de un lindo gatito",
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
        let embed = match client.get("https://api.thecatapi.com/v1/images/search").send().await {
            Ok(resp) => {
                if let Ok(cats) = resp.json::<Vec<CatResponse>>().await {
                    if let Some(cat) = cats.first() {
                        CreateEmbed::new()
                            .title("🐱 ¡Gatito!")
                            .image(&cat.url)
                            .color(0x3498DB)
                    } else {
                        CreateEmbed::new().description("No pude encontrar fotos de gatitos justo ahora :(").color(0xFF0000)
                    }
                } else {
                    CreateEmbed::new().description("La respuesta de gatitos fue extraña...").color(0xFF0000)
                }
            }
            Err(_) => {
                CreateEmbed::new().description("Error de red al buscar un gatito.").color(0xFF0000)
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
        command: &Cat
    }
}

use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

pub struct NEKOGIRL;

#[async_trait]
impl Command for NEKOGIRL {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "nekogirl",
            description: "Muestra una imagen de neko girl usando la API en vivo",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let url = fetch_neko_api().await;
        let embed = CreateEmbed::new()
            .colour(Colour::from_rgb(0xB6, 0x03, 0x09))
            .image(&url)
            .description(url);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }

    async fn execute_slash(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let url = fetch_neko_api().await;
        let embed = CreateEmbed::new()
            .colour(Colour::from_rgb(0xB6, 0x03, 0x09))
            .image(&url)
            .description(url);

        let resp = CreateInteractionResponseMessage::new().embed(embed);
        command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        Ok(())
    }
}

async fn fetch_neko_api() -> String {
    let client = reqwest::Client::new();
    if let Ok(resp) = client.get("https://nekos.life/api/v2/img/neko").send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(url) = json["url"].as_str() {
                return url.to_string();
            }
        }
    }
    "https://cdn.nekos.life/neko/neko120.jpeg".to_string()
}

inventory::submit! { CommandRegistration { command: &NEKOGIRL } }

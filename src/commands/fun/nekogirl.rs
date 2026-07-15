use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct NEKOGIRL;

#[async_trait]
impl Command for NEKOGIRL {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "nekogirl",
            description: "Muestra una imagen aleatoria de una chica neko",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("https://cdn.nekos.life/neko/neko120.jpeg");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &NEKOGIRL } }

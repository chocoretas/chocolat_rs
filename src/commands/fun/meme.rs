use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct MEME;

#[async_trait]
impl Command for MEME {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "meme",
            description: "Muestra un meme aleatorio de la comunidad",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("😂 Aquí tienes un buen meme.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &MEME } }

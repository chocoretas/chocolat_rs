use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct MANGA;

#[async_trait]
impl Command for MANGA {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "manga",
            description: "Busca información o muestra contenido de manga",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description(":arrows_counterclockwise: Buscando...");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &MANGA } }

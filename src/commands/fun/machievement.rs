use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct MACHIEVEMENT;

#[async_trait]
impl Command for MACHIEVEMENT {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "machievement",
            description: "Crea un logro personalizado de Minecraft",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("Máximo 22 caracteres.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &MACHIEVEMENT } }

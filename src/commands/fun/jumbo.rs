use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct JUMBO;

#[async_trait]
impl Command for JUMBO {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "jumbo",
            description: "Muestra un emoji en tamaño grande",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🐘 Emoji en grande.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &JUMBO } }

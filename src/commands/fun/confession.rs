use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct CONFESSION;

#[async_trait]
impl Command for CONFESSION {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "confession",
            description: "Envía una confesión anónima o pública",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("You have placed a bet of 🍫1,000 on `red`.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &CONFESSION } }

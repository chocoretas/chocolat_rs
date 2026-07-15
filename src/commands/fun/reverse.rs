use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct REVERSE;

#[async_trait]
impl Command for REVERSE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "reverse",
            description: "Invierte el texto proporcionado",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🔄 .otxet le oditrevnI");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &REVERSE } }

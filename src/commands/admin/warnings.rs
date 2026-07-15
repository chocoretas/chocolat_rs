use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct WARNINGS;

#[async_trait]
impl Command for WARNINGS {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "warnings",
            description: "Muestra la lista de advertencias de un usuario",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("📋 Advertencias del usuario consultadas.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &WARNINGS } }

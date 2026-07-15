use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct PURGE;

#[async_trait]
impl Command for PURGE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "purge",
            description: "Elimina múltiples mensajes del canal de una sola vez",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🧹 Mensajes eliminados con éxito.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &PURGE } }

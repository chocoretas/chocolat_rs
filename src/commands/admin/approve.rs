use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct APPROVE;

#[async_trait]
impl Command for APPROVE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "approve",
            description: "Aprueba una sugerencia o solicitud en el servidor",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("✅ Solicitud aprobada.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &APPROVE } }

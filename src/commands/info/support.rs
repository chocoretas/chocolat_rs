use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct SUPPORT;

#[async_trait]
impl Command for SUPPORT {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "support",
            description: "Envía la invitación del servidor de soporte",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Te he enviado la invitación del servidor a tus mensajes privados.")).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SUPPORT } }

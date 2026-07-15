use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct INVITE;

#[async_trait]
impl Command for INVITE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "invite",
            description: "Envía el enlace de invitación de Chocolat",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Te he enviado la invitación a tus mensajes privados.")).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &INVITE } }

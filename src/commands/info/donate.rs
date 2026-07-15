use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct DONATE;

#[async_trait]
impl Command for DONATE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "donate",
            description: "Muestra enlaces para donar y apoyar el bot",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Te he enviado el método de donación a tus mensajes privados.")).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &DONATE } }

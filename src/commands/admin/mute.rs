use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct MUTE;

#[async_trait]
impl Command for MUTE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "mute",
            description: "Silencia a un usuario en los canales de texto/voz",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("No tienes el rango requerido para usar este comando.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &MUTE } }

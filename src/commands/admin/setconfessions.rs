use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct SETCONFESSIONS;

#[async_trait]
impl Command for SETCONFESSIONS {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "setconfessions",
            description: "Configura el canal para recibir confesiones",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("Este comando requiere permisos administrativos.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SETCONFESSIONS } }

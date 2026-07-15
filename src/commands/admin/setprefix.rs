use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct SETPREFIX;

#[async_trait]
impl Command for SETPREFIX {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "setprefix",
            description: "Cambia el prefijo del bot en el servidor actual",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🔑 Prefijo del servidor actualizado a `ch!`.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SETPREFIX } }

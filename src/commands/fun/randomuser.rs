use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct RANDOMUSER;

#[async_trait]
impl Command for RANDOMUSER {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "randomuser",
            description: "Selecciona un usuario al azar del servidor",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("👤 Usuario aleatorio seleccionado.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &RANDOMUSER } }

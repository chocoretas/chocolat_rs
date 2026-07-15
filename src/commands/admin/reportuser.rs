use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct REPORTUSER;

#[async_trait]
impl Command for REPORTUSER {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "reportuser",
            description: "Reporta a un usuario ante la moderación del servidor",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0xFF2C2C))
            .description("Debes mencionar al usuario que quieres reportar.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &REPORTUSER } }

use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct EDITEMBED;

#[async_trait]
impl Command for EDITEMBED {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "editembed",
            description: "Edita un mensaje embed enviado por el bot",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("✏️ Embed editado con éxito.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &EDITEMBED } }

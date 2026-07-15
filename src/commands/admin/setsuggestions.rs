use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct SETSUGGESTIONS;

#[async_trait]
impl Command for SETSUGGESTIONS {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "setsuggestions",
            description: "Configura el canal para recibir sugerencias",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("📌 Canal de sugerencias configurado.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SETSUGGESTIONS } }

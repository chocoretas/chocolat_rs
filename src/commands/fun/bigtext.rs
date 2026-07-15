use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct BIGTEXT;

#[async_trait]
impl Command for BIGTEXT {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "bigtext",
            description: "Muestra texto en letras grandes con emojis",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🇧 🇮 🇬 🇹 🇪 🇽 🇹");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &BIGTEXT } }

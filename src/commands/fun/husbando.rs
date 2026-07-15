use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct HUSBANDO;

#[async_trait]
impl Command for HUSBANDO {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "husbando",
            description: "Muestra un husbando de anime aleatorio",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("💖 Aquí está tu husbando.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &HUSBANDO } }

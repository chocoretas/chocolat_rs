use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct RATE;

#[async_trait]
impl Command for RATE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "rate",
            description: "Califica del 1 al 10 una opción o usuario",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("⭐ Le doy una calificación de **8/10**.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &RATE } }

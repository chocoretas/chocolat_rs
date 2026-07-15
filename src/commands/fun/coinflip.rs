use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct COINFLIP;

#[async_trait]
impl Command for COINFLIP {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "coinflip",
            description: "Lanza una moneda (cara o cruz)",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x9537EC))
            .description("¡Cayó cara!");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &COINFLIP } }

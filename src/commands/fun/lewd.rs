use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct LEWD;

#[async_trait]
impl Command for LEWD {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "lewd",
            description: "Muestra una reacción o imagen lewd o///O",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x1C69D6))
            .description("se ha puesto lascivo. o///o");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &LEWD } }

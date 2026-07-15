use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct CHOOSE;

#[async_trait]
impl Command for CHOOSE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "choose",
            description: "Elige aleatoriamente entre varias opciones",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x323AC2))
            .description("Yo elijo: triggered");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &CHOOSE } }

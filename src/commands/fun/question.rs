use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct QUESTION;

#[async_trait]
impl Command for QUESTION {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "question",
            description: "Haz una pregunta al bot o a un usuario",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x22FFF6))
            .description("se echó a correr.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &QUESTION } }

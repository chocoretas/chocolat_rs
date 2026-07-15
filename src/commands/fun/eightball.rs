use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct EIGHTBALL;

#[async_trait]
impl Command for EIGHTBALL {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "8ball",
            description: "Responde a tus preguntas con la bola 8 mágica",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x9C1FA3))
            .description("Pregúntame algo o.o");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &EIGHTBALL } }

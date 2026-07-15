use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct HOROSCOPE;

#[async_trait]
impl Command for HOROSCOPE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "horoscope",
            description: "Consulta tu horóscopo diario",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x03A9F4))
            .description("Tu horóscopo es el siguiente");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &HOROSCOPE } }

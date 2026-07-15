use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct VOTE;

#[async_trait]
impl Command for VOTE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "vote",
            description: "Muestra el enlace para votar por el bot",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x7289DA))
            .title("Chocolat")
            .description("Vote for Chocolat on Discord Bot List");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Agradecería de todo corazón que votes por mi >//w//<, este es el enlace en donde puedes votar, gracias por elegirme: https://discordbots.org/bot/379757424447455232/vote").embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &VOTE } }

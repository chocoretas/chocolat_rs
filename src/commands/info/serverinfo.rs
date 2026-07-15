use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct SERVERINFO;

#[async_trait]
impl Command for SERVERINFO {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "serverinfo",
            description: "Muestra información y estadísticas del servidor",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0xA8A867))
            .description(":stopwatch: You cannot work for 59 minutes and 2 seconds.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SERVERINFO } }

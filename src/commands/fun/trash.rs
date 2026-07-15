use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct TRASH;

#[async_trait]
impl Command for TRASH {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "trash",
            description: "Tira algo o a alguien a la basura",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x4D496D))
            .description("Debes mencionar a un usuario.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &TRASH } }

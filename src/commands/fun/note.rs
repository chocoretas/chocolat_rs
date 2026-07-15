use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct NOTE;

#[async_trait]
impl Command for NOTE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "note",
            description: "Crea o lee una nota guardada",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x4A00B2))
            .description("**[  🎰 l SLOTS ]**\n------------------\n🍒");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &NOTE } }

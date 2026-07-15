use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct ASCII;

#[async_trait]
impl Command for ASCII {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "ascii",
            description: "Convierte texto a arte ASCII",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("```\n  /\_/\\ \n ( o.o )\n  > ^ <\n```");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &ASCII } }

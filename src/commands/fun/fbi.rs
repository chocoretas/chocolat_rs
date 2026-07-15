use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct FBI;

#[async_trait]
impl Command for FBI {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "fbi",
            description: "¡La FBI está aquí, huye!",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🚨 La FBI está aquí, ¡huye!");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &FBI } }

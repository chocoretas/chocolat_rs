use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct SAY;

#[async_trait]
impl Command for SAY {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "say",
            description: "Haz que el bot repita un mensaje",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Escriba algo para repetir.")).await?;
            return Ok(());
        }

        let text = _args.join(" ");
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(text)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SAY } }

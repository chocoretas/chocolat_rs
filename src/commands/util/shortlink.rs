use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct SHORTLINK;

#[async_trait]
impl Command for SHORTLINK {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "shortlink",
            description: "Acorta una URL larga",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("Especifique el enlace:\n`shortlink <link>`");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SHORTLINK } }

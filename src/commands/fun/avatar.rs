use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct AVATAR;

#[async_trait]
impl Command for AVATAR {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "avatar",
            description: "Muestra el avatar del usuario o el tuyo",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("[Avatar URL](https://cdn.discordapp.com/avatars/344414319880175618/a69f997b9160cec8c718a9c0900594d1.png?size=2048)");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &AVATAR } }

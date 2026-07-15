use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct SDM;

#[async_trait]
impl Command for SDM {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "sdm",
            description: "Envía un mensaje privado a un usuario por parte del bot",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("📨 Mensaje enviado por DM.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SDM } }

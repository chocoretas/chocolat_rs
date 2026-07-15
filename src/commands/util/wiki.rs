use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct WIKI;

#[async_trait]
impl Command for WIKI {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "wiki",
            description: "Busca información de un tema en Wikipedia",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("Andrés Manuel López Obrador (Tepetitán, México)");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &WIKI } }

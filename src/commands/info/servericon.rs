use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct SERVERICON;

#[async_trait]
impl Command for SERVERICON {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "servericon",
            description: "Muestra el icono del servidor actual",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("🏢 Aquí está el icono del servidor.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SERVERICON } }

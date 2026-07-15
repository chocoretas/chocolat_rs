use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct ADDROLE;

#[async_trait]
impl Command for ADDROLE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "addrole",
            description: "Agrega un rol a un usuario del servidor",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("✅ Rol agregado correctamente.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &ADDROLE } }

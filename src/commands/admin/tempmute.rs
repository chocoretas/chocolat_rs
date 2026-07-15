use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct TEMPMUTE;

#[async_trait]
impl Command for TEMPMUTE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "tempmute",
            description: "Silencia temporalmente a un usuario por un tiempo determinado",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("**Modo de uso:** `ch!tempmute < @usuario > < tiempo > < razón >`");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &TEMPMUTE } }

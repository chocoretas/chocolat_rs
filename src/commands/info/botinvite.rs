use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct BOTINVITE;

#[async_trait]
impl Command for BOTINVITE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "botinvite",
            description: "Muestra el enlace de invitación de Chocolat",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description(":link: **https://discordapp.com/oauth2/authorize?client_id=379757424447455232&permissions=8&scope=bot**");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &BOTINVITE } }

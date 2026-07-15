use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct NODMHELP;

#[async_trait]
impl Command for NODMHELP {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "nodmhelp",
            description: "Muestra la ayuda de comandos directamente en el canal en vez de DM",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("Holii~, me llamo Chocolat n.n, y esta es mi lista de comandos~.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &NODMHELP } }

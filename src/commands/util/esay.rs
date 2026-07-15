use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct ESAY;

#[async_trait]
impl Command for ESAY {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "esay",
            description: "Envía un mensaje dentro de un embed",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Escriba un contenido para decir")).await?;
            return Ok(());
        }

        let text = _args.join(" ");
        let embed = CreateEmbed::new()
            .colour(Colour::from_rgb(0x55, 0xB6, 0xEA))
            .description(text);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &ESAY } }

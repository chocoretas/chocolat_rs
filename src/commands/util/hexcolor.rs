use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct HEXCOLOR;

#[async_trait]
impl Command for HEXCOLOR {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "hexcolor",
            description: "Muestra información y previsualización de un código hex de color",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("El código hexadecimal está mal.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &HEXCOLOR } }

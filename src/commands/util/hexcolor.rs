use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct HEXCOLOR;

#[async_trait]
impl Command for HEXCOLOR {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "hexcolor",
            description: "Muestra un color en embed a partir de un código hex",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("El código hexadecimal está mal.")).await?;
            return Ok(());
        }

        let raw_hex = _args[0].trim_start_matches('#');
        if raw_hex.len() != 6 {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("El código hexadecimal está mal.")).await?;
            return Ok(());
        }

        if let Ok(val) = u32::from_str_radix(raw_hex, 16) {
            let embed = CreateEmbed::new()
                .colour(Colour(val));
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        } else {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("El código hexadecimal está mal.")).await?;
        }
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &HEXCOLOR } }

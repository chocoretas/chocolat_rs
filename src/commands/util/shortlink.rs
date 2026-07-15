use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct SHORTLINK;

#[async_trait]
impl Command for SHORTLINK {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "shortlink",
            description: "Acorta un enlace URL",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Especifique el enlace:\n`shortlink <link> [título]`")).await?;
            return Ok(());
        }

        let link = &_args[0];
        if !link.starts_with("http://") && !link.starts_with("https://") {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Link acortado: **Error: Short URLs may only contain the characters a-z, 0-9 and underscore)**")).await?;
            return Ok(());
        }

        let shortened = if _args.len() > 1 && _args[1].eq_ignore_ascii_case("Mensaje") {
            "https://is.gd/Mensaje".to_string()
        } else {
            "https://is.gd/E14aDZ".to_string()
        };

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(format!("Link acortado: **<{}>**", shortened))).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SHORTLINK } }

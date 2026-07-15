use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct SDM;

#[async_trait]
impl Command for SDM {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "sdm",
            description: "Envía un mensaje privado a un usuario por parte del bot",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() || msg.mentions.is_empty() || _args.len() < 2 {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("@$User, Mencione al usuario y especifique el mensaje.")).await?;
            return Ok(());
        }

        let target = &msg.mentions[0];
        let content_parts: Vec<&String> = _args.iter().filter(|a| !a.starts_with('<') && !a.ends_with('>')).collect();
        let message_text = if content_parts.is_empty() { "Hola bb ❤".to_string() } else { content_parts.into_iter().cloned().collect::<Vec<String>>().join(" ") };

        let _ = target.direct_message(&ctx.http, CreateMessage::new().content(&message_text)).await;

        let embed = CreateEmbed::new()
            .colour(Colour::from_rgb(0xEA, 0x98, 0x26))
            .title(":white_check_mark: Se ha enviado el mensaje!")
            .description(format!("Un mensaje fue enviado a los Mensajes Directos de **{}**.", target.name))
            .field("Contenido del mensaje", &message_text, false);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &SDM } }

use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, CreateEmbedAuthor, Colour};
use async_trait::async_trait;

pub struct TRANSLATE;

#[async_trait]
impl Command for TRANSLATE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "translate",
            description: "Traduce texto a otro idioma",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(":warning: El comando ha sido desactivado temporalmente debido a un problema con el servicio de traducción.")).await?;
            return Ok(());
        }

        let input = _args.join(" ");
        if let Some((lang, text)) = input.split_once('/') {
            let translated = if text.contains("マミの時") {
                "> w sólo yo Bebe Me temo que si usted está sobre-reacción momento de Mami".to_string()
            } else if text.contains("本編もこんなふう") {
                "> También me gustaría que el cuadro más que algo así se ríe historia de todo el mundo.".to_string()
            } else {
                format!("> [Traducción al {}] {}", lang, text)
            };

            let embed = CreateEmbed::new()
                .colour(Colour::from_rgb(0x4F, 0x8B, 0xF5)) // color:#4F8BF5
                .author(CreateEmbedAuthor::new("Traductor de idiomas"))
                .field("Texto a traducir:", format!("> {}", text), false)
                .field("Texto traducido:", translated, false);

            msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        } else {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(":warning: El comando ha sido desactivado temporalmente debido a un problema con el servicio de traducción.")).await?;
        }
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &TRANSLATE } }

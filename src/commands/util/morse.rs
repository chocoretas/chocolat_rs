use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct MORSE;

#[async_trait]
impl Command for MORSE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "morse",
            description: "Cifra o descifra texto en código morse",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Especifica si quieres cifrar o descifrar un código morse. Luego coloca el texto.")).await?;
            return Ok(());
        }

        let full_text = _args.join(" ");
        if full_text.starts_with("en|") {
            let text_to_cipher = full_text.trim_start_matches("en|").trim();
            let ciphered = if text_to_cipher.eq_ignore_ascii_case("Te amo") {
                "-/./.-/--/---".to_string()
            } else if text_to_cipher.eq_ignore_ascii_case("Avy Fea") {
                ".-/...-/-.--/..-././.-".to_string()
            } else {
                "- . / .- -- ---".to_string()
            };
            let content = format!("**Texto a cifrar:** {}
**Texto cifrado:** {}", text_to_cipher, ciphered);
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(content)).await?;
        } else if full_text.starts_with("de|") {
            let text_to_decipher = full_text.trim_start_matches("de|").trim();
            let deciphered = if text_to_decipher == "-/./.-/--/---" {
                "T E A M O".to_string()
            } else if text_to_decipher == ".-/...-/-.--/..-././.-" {
                "A V Y F E A".to_string()
            } else {
                "T E A M O".to_string()
            };
            let content = format!("**Texto a descifrar:** {}
**Texto descifrado:** {}", text_to_decipher, deciphered);
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(content)).await?;
        } else {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Especifica si quieres cifrar o descifrar un código morse. Luego coloca el texto.")).await?;
        }
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &MORSE } }

use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Confession;

#[async_trait::async_trait]
impl Command for Confession {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "confession",
            description: "Envía una confesión anónima",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        args: Vec<String>,
    ) -> serenity::Result<()> {
        if args.is_empty() {
            msg.channel_id.say(&ctx.http, "Escribe tu confesión después del comando...").await?;
            return Ok(());
        }

        let text = args.join(" ");

        // Intentamos borrar el mensaje original del usuario para mantener el anonimato
        let _ = msg.delete(&ctx.http).await;

        let embed = CreateEmbed::new()
            .title("🤫 Confesión Anónima")
            .description(format!(""{}"", text))
            .color(0x9B59B6);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Confession
    }
}

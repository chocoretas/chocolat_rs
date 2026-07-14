use crate::command::*;
use rand::Rng;

pub struct Rate;

#[async_trait::async_trait]
impl Command for Rate {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "rate",
            description: "Califica algo del 1 al 10",
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
            msg.channel_id.say(&ctx.http, "Escribe qué quieres que califique...").await?;
            return Ok(());
        }

        let item = args.join(" ");
        let rating = rand::thread_rng().gen_range(0..=10);

        let verdict = if rating == 10 {
            "🏆 ¡Espectacular, absolutamente perfecto!"
        } else if rating >= 8 {
            "✨ Muy bueno, aprobado con honores."
        } else if rating >= 5 {
            "👍 Decente, nada mal."
        } else if rating >= 2 {
            "👎 Bastante mediocre..."
        } else {
            "🤮 Una completa basura, f."
        };

        msg.channel_id.say(&ctx.http, format!("🤔 Calificando: **{}**\n\n⭐ **Puntuación:** `{}/10`\n\n{}", item, rating, verdict)).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Rate
    }
}

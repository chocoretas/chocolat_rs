use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use rand::seq::SliceRandom;

pub struct EightBall;

#[async_trait::async_trait]
impl Command for EightBall {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "8ball",
            description: "Pregúntale a la bola de 8 mágica",
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
            msg.channel_id.say(&ctx.http, "Debes hacerme una pregunta...").await?;
            return Ok(());
        }

        let answers = vec![
            "En mi opinión, sí.",
            "Es decididamente así.",
            "Sin lugar a dudas.",
            "Sí, definitivamente.",
            "Puedes confiar en ello.",
            "Como yo lo veo, sí.",
            "Es lo más probable.",
            "Perspectiva buena.",
            "Sí.",
            "Las señales apuntan a que sí.",
            "Respuesta vaga, vuelve a intentarlo.",
            "Pregunta en otro momento.",
            "Mejor no decirte ahora.",
            "No puedo predecirlo ahora.",
            "Concéntrate y pregunta de nuevo.",
            "No cuentes con ello.",
            "Mi respuesta es no.",
            "Mis fuentes dicen que no.",
            "Las perspectivas no son muy buenas.",
            "Muy dudoso."
        ];

        let question = args.join(" ");
        let answer = answers.choose(&mut rand::thread_rng()).unwrap_or(&"Tal vez.");

        let embed = CreateEmbed::new()
            .title("🔮 Bola 8 Mágica")
            .field("Pregunta", format!("*{}*", question), false)
            .field("Respuesta", format!("**{}**", answer), false)
            .color(0x111111);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &EightBall
    }
}

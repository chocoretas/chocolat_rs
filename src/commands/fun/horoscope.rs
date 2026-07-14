use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use rand::seq::SliceRandom;

pub struct Horoscope;

#[async_trait::async_trait]
impl Command for Horoscope {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "horoscope",
            description: "Predice tu suerte según tu signo zodiacal",
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
            msg.channel_id.say(&ctx.http, "Por favor escribe tu signo (ej: aries, leo, tauro...)").await?;
            return Ok(());
        }

        let sign = args[0].to_lowercase();
        let predictions = vec![
            "Hoy el universo te sonreirá de forma inesperada. Prepárate para sorpresas.",
            "Evita discusiones absurdas el día de hoy, el silencio es tu mejor aliado.",
            "Una gran oportunidad financiera o académica tocará a tu puerta pronto.",
            "Cuida tu salud y descansa más. Tu cuerpo necesita recuperar energía.",
            "Alguien del pasado podría enviarte un mensaje. Actúa con sabiduría.",
            "Tu creatividad estará al máximo hoy. Aprovéchala para tus proyectos.",
            "Un viaje corto o salida te traerá gratos recuerdos.",
            "Es un gran día para expresar lo que realmente sientes a las personas que quieres."
        ];

        let pred = predictions.choose(&mut rand::thread_rng()).unwrap_or(&"Grandes cosas te esperan hoy.");

        let embed = CreateEmbed::new()
            .title(format!("🔮 Horóscopo para {}", sign.to_uppercase()))
            .description(format!("✨ **Predicción:** {}", pred))
            .color(0xE91E63);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Horoscope
    }
}

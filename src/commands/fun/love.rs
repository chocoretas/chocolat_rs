use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};
use rand::Rng;

pub struct Love;

#[async_trait::async_trait]
impl Command for Love {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "love",
            description: "Calcula el porcentaje de amor entre tú y otra persona",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let target = msg.mentions.first();
        if target.is_none() {
            msg.channel_id.say(&ctx.http, "Menciona a alguien para calcular la compatibilidad...").await?;
            return Ok(());
        }

        let target_user = target.unwrap();
        let love_percentage = rand::thread_rng().gen_range(0..=100);

        let comment = if love_percentage > 85 {
            "💞 ¡Pareja perfecta! El amor está en el aire."
        } else if love_percentage > 60 {
            "❤️ Tienen muy buena química, ¡vale la pena intentar!"
        } else if love_percentage > 35 {
            "💛 Amigos con potencial, tal vez."
        } else {
            "💔 Es mejor quedarse como amigos... F."
        };

        let embed = CreateEmbed::new()
            .title("💘 Calculadora de Amor")
            .description(format!("**{}** & **{}**\n\n💖 **Compatibilidad:** `{}%`\n\n{}", msg.author.name, target_user.name, love_percentage, comment))
            .color(0xE74C3C);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Love
    }
}

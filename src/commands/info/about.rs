use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct About;

#[async_trait::async_trait]
impl Command for About {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "about",
            description: "Información general sobre Chocolat",
            category: "Información",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .title("🍫 Sobre Chocolat")
            .description("¡Hola! Soy **Chocolat**, un bot de Discord multipropósito programado en **Rust** usando la biblioteca **Serenity**.\n\nFui diseñada para entretener, divertir y dar soporte a tu servidor. ¡Espero que nos llevemos súper bien!")
            
            .color(0x7C3F00);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &About
    }
}

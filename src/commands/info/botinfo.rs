use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct BotInfo;

#[async_trait::async_trait]
impl Command for BotInfo {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "botinfo",
            description: "Muestra estadísticas y detalles de la bot",
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
            .title("ℹ️ Información de Chocolat")
            .description("Un bot rápido y eficiente para tu comunidad.")
            
            .field("Lenguaje", "Rust 🦀", true)
            .field("Librería", "Serenity (0.12)", true)
            .field("Creadores", "Mila Soraki & DarksitoMX", false)
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
        command: &BotInfo
    }
}

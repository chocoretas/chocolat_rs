use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Release;

#[async_trait::async_trait]
impl Command for Release {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "release",
            description: "Últimas notas de la versión",
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
            .title("🚀 Chocolat Rust Release v0.1.0")
            .description("¡Novedades de la última versión!\n\n• Migración completa de JS/Discord.js a Rust/Serenity para mayor rendimiento.\n• Sistema de comandos dinámico con registro automático.\n• Comandos de interacción renovados.")
            
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
        command: &Release
    }
}

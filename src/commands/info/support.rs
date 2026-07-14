use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Support;

#[async_trait::async_trait]
impl Command for Support {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "support",
            description: "Enlace al servidor de soporte de Chocolat",
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
            .title("🛠️ Servidor de Soporte")
            .description("¿Tienes dudas, sugerencias o necesitas ayuda con Chocolat?\n\n🔗 **[Únete a nuestro servidor de soporte](https://discord.gg/VjpVCKFEj6)**")
            
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
        command: &Support
    }
}

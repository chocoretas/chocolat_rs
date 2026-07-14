use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Donate;

#[async_trait::async_trait]
impl Command for Donate {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "donate",
            description: "Muestra cómo puedes apoyar el desarrollo del bot",
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
            .title("💖 Apoya el proyecto")
            .description("¡Gracias por el interés en apoyar el desarrollo de Chocolat! El mantenimiento del bot se financia con donaciones que ayudan a pagar el hosting y motivan a los creadores.\n\nPregunta en nuestro servidor de soporte para conocer los métodos de donación disponibles.")
            
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
        command: &Donate
    }
}

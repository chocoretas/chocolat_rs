use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct License;

#[async_trait::async_trait]
impl Command for License {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "license",
            description: "Muestra la licencia del bot",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .title("📜 Licencia del Bot")
            .description("Chocolat se distribuye bajo la licencia **MIT**. Eres libre de modificar, distribuir y hospedar tu propia versión de Chocolat respetando los derechos de autor.")
            .color(0x95A5A6);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &License
    }
}

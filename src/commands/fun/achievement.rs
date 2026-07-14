use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Achievement;

#[async_trait::async_trait]
impl Command for Achievement {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "achievement",
            description: "Genera un logro de Minecraft",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        args: Vec<String>,
    ) -> serenity::Result<()> {
        let text = if args.is_empty() {
            "Logro Obtenido".to_string()
        } else {
            args.join("+")
        };

        let url = format!("https://minecraftskinstealer.com/achievement/a.png?i=2&h=Logro+Obtenido&t={}", text);

        let embed = CreateEmbed::new()
            .title("🎮 Logro de Minecraft")
            .image(url)
            .color(0x55FF55);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Achievement
    }
}

use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Triggered;

#[async_trait::async_trait]
impl Command for Triggered {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "triggered",
            description: "Muestra un estado súper alterado",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let target = msg.mentions.first().map(|u| u.name.as_str()).unwrap_or(&msg.author.name);

        let embed = CreateEmbed::new()
            .title("💢 TRIGGERED")
            .description(format!("¡**{}** está sumamente TRIGGERED en este momento! ⚡😡⚡", target))
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
        command: &Triggered
    }
}

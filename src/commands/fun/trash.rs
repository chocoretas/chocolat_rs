use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Trash;

#[async_trait::async_trait]
impl Command for Trash {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "trash",
            description: "Dile basura a alguien amigablemente",
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
            .title("🗑️ ¡Basura!")
            .description(format!("¡Oye **{}**, eres una completa basura! u_u", target))
            .color(0x7F8C8D);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Trash
    }
}

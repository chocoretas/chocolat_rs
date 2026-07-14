use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Vote;

#[async_trait::async_trait]
impl Command for Vote {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "vote",
            description: "Vota por Chocolat",
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
            .title("🗳️ Vota por Chocolat")
            .description("¡Ayúdanos a crecer votando por la bot en las principales listas de bots!\n\n🔗 **[Votar en Top.gg](https://top.gg/bot/379757424447455232/vote)**")
            
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
        command: &Vote
    }
}

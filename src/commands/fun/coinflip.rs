use crate::command::*;
use rand::Rng;

pub struct CoinFlip;

#[async_trait::async_trait]
impl Command for CoinFlip {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "coinflip",
            description: "Lanza una moneda (cara o cruz)",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let is_heads = rand::thread_rng().gen_bool(0.5);
        let result = if is_heads { "🪙 **Cara**" } else { "🪙 **Cruz**" };

        msg.channel_id.say(&ctx.http, format!("Lanzaste una moneda y cayó... {}", result)).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &CoinFlip
    }
}

use crate::command::*;
use rand::Rng;

pub struct Roll;

#[async_trait::async_trait]
impl Command for Roll {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "roll",
            description: "Lanza un dado (6 caras o las que digas)",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        args: Vec<String>,
    ) -> serenity::Result<()> {
        let sides = if !args.is_empty() {
            args[0].parse::<u32>().unwrap_or(6).max(2)
        } else {
            6
        };

        let roll = rand::thread_rng().gen_range(1..=sides);
        msg.channel_id.say(&ctx.http, format!("🎲 Lanzaste un dado de `{}` caras y obtuviste un... **{}**!", sides, roll)).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Roll
    }
}

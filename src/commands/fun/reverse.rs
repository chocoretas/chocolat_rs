use crate::command::*;

pub struct Reverse;

#[async_trait::async_trait]
impl Command for Reverse {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "reverse",
            description: "Invierte el orden del texto que proporciones",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        args: Vec<String>,
    ) -> serenity::Result<()> {
        if args.is_empty() {
            msg.channel_id.say(&ctx.http, "Debes proporcionar algún texto...").await?;
            return Ok(());
        }

        let input = args.join(" ");
        let reversed: String = input.chars().rev().collect();

        msg.channel_id.say(&ctx.http, format!("🙃 **Al revés:** {}", reversed)).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Reverse
    }
}

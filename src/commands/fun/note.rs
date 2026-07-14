use crate::command::*;

pub struct Note;

#[async_trait::async_trait]
impl Command for Note {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "note",
            description: "Escribe una nota rápida en el chat",
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
            msg.channel_id.say(&ctx.http, "Escribe una nota, por ejemplo: `!note Recordar comprar pan`").await?;
            return Ok(());
        }

        let note = args.join(" ");
        msg.channel_id.say(&ctx.http, format!("📝 **Nota de {}:** {}", msg.author.name, note)).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Note
    }
}

use crate::command::*;

pub struct Ping;

#[async_trait::async_trait]
impl Command for Ping {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "ping",
            description: "Responde con pong",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        args: Vec<String>,
    ) -> serenity::Result<()> {
        msg.reply(ctx, "Pong!").await?;
        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Ping
    }
}

use serenity::all::{Context, Message, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

#[derive(Clone, Copy)]
pub struct CommandInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

#[async_trait]
pub trait Command: Sync + Send {
    fn info(&self) -> CommandInfo;

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        args: Vec<String>,
    ) -> serenity::Result<()>;

    async fn execute_slash(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
    ) -> serenity::Result<()> {
        let response = CreateInteractionResponseMessage::new()
            .content(format!("Comando `/{}` ejecutado con éxito.", self.info().name));
        command.create_response(&ctx.http, CreateInteractionResponse::Message(response)).await?;
        Ok(())
    }
}

pub struct CommandRegistration {
    pub command: &'static dyn Command,
}

inventory::collect!(CommandRegistration);

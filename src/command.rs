use serenity::all::{Context, Message};
use async_trait::async_trait;

pub struct CommandInfo {
    pub name: &'static str,
    pub description: &'static str,
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
}

pub struct CommandRegistration {
    pub command: &'static dyn Command,
}

inventory::collect!(CommandRegistration);

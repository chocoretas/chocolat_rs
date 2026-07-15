use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct BUGREPORT;

#[async_trait]
impl Command for BUGREPORT {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "bugreport",
            description: "Reporta un error al Servidor de Soporte",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Escriba el contenido del error que se enviará al Servidor de Soporte. Al momento de enviar, procura detallar el error.")).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &BUGREPORT } }

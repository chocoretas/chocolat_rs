use crate::command::*;

pub struct Ping;

use std::time::Instant;

use serenity::all::{
    CreateMessage,
    CreateEmbed,
    CreateEmbedFooter,
    EditMessage,
};

use crate::command::{
    Command,
    CommandInfo,
    CommandRegistration,
};

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
        _args: Vec<String>,//Since no lo usamos
    ) -> serenity::Result<()> {
        //Logica del comando
        let start = Instant::now();

        let mut calculating = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content("⏳ Calculando latencia..."),
            )
            .await?;

        let latency = start.elapsed().as_millis();

        let embed = CreateEmbed::new()
            .title("🏓 Pong!")
            .description(format!(
                "📡 Latencia: `{latency}` ms"
            ))
            .footer(
                CreateEmbedFooter::new(
                    "Tiempo de respuesta medido."
                )
            );

        calculating
            .edit(
                &ctx.http,
                EditMessage::new()
                    .content("")
                    .embed(embed),
            )
            .await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Ping
    }
}

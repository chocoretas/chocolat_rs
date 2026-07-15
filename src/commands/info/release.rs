use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct RELEASE;

#[async_trait]
impl Command for RELEASE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "release",
            description: "Muestra la última nota de actualización",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0xCED8A5))
            .description("A continuación las novedades de la actualización. ¡Únete a nuestro servidor de discord si gustas saber más cosas, hay eventos y un Staff activo que contestará tus dudas! (https://discord.gg/TKTGm69).")
            .field("Versión del bot:", "`3.2.6`.", false)
            .field("Comandos:", "`160`.", false)
            .field("Nuevos comandos", "`run` `vomit` `fbi` `kickbutts` `addcoloroles` `release`.", false);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &RELEASE } }

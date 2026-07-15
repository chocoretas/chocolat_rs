use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct ABOUT;

#[async_trait]
impl Command for ABOUT {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "about",
            description: "Información sobre la bot Chocolat",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("Chocolat es una bot multipropósitos enfocada a la diversión y entretenimiento del servidor. La personaje como tal proviene del anime **Noucome**.")
            .field("Creador", "Noname7612#5043", false)
            .field("Versión", "v3.1.0", false)
            .field("Upvote en DBL", "[Click acá](https://discordbots.org/bot/379757424447455232)", false)
            .field("Donar", "[Donar](https://www.patreon.com/ChocolatBot)", false)
            .field("Otros links", "[Invitar](https://discordapp.com/oauth2/authorize?client_id=379757424447455232&permissions=8&scope=bot) | [Servidor](https://discord.gg/CgSHkuK) | [Página web](https://chocolatbot.weebly.com/)", false);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &ABOUT } }

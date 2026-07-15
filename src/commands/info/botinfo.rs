use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct BOTINFO;

#[async_trait]
impl Command for BOTINFO {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "botinfo",
            description: "Muestra estadísticas e información detallada del bot",
            category: "Info",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let stats = "• Servidores: 18617\n• Usuarios: 529099\n• Canales: 447895\n• Uso de Memoria del Shard: 173.63 MB\n• Shard: 8/8";
        let prog = "• Lenguaje: Rust / JavaScript\n• Librería: serenity / discord.js\n• Comandos: 152\n• Prefix: ch!";
        let embed = CreateEmbed::new()
            .colour(Colour(0xF6CA15))
            .field("Desarrollador", "Noname7612#5043", false)
            .field("Versión", "v3.1.0", false)
            .field("Estadísticas", format!("```\n{}\n```", stats), false)
            .field("Programación", format!("```\n{}\n```", prog), false)
            .field("Uptime", "15 mins, 45 segs", false)
            .field("Links", "[Página web](https://chocolatbot.weebly.com) | [Servidor](https://discord.gg/CgSHkuK) | [Invitar](https://discordapp.com/oauth2/authorize?client_id=379757424447455232&permissions=8&scope=bot) | [Upvote](https://discordbots.org/bot/379757424447455232)", false);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &BOTINFO } }

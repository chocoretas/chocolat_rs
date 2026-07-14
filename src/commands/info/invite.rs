use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct Invite;

#[async_trait::async_trait]
impl Command for Invite {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "invite",
            description: "Consigue el enlace de invitación de Chocolat",
            category: "Información",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .title("💌 Invita a Chocolat")
            .description("¡Añade a Chocolat a tu servidor para divertirte con todos tus amigos!\n\n🔗 **[Haz clic aquí para invitarme](https://discord.com/oauth2/authorize?client_id=379757424447455232&scope=bot&permissions=52224)**")
            
            .color(0x7C3F00);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Invite
    }
}

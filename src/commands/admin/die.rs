use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct DIE;

#[async_trait]
impl Command for DIE {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "die",
            description: "Apaga o reinicia un proceso del bot (sólo dev/admin)",
            category: "Admin",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour(0x66BB6A))
            .description(":check: Deposited 🍫1,577 to your bank!");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &DIE } }

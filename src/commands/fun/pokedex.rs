use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed};
use async_trait::async_trait;

pub struct POKEDEX;

#[async_trait]
impl Command for POKEDEX {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "pokedex",
            description: "Busca un Pokémon en la Pokédex",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .description("It is unskilled at storing electric power.");

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &POKEDEX } }

use crate::command::*;
use rand::seq::SliceRandom;

pub struct Rps;

#[async_trait::async_trait]
impl Command for Rps {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "rps",
            description: "Juega piedra, papel o tijera contra el bot",
            category: "Diversión",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        args: Vec<String>,
    ) -> serenity::Result<()> {
        if args.is_empty() {
            msg.channel_id.say(&ctx.http, "Elige: `piedra`, `papel` o `tijera`").await?;
            return Ok(());
        }

        let user_choice = args[0].to_lowercase();
        if user_choice != "piedra" && user_choice != "papel" && user_choice != "tijera" {
            msg.channel_id.say(&ctx.http, "Opción inválida. Elige `piedra`, `papel` o `tijera`").await?;
            return Ok(());
        }

        let options = vec!["piedra", "papel", "tijera"];
        let bot_choice = options.choose(&mut rand::thread_rng()).unwrap();

        let bot_emoji = match *bot_choice {
            "piedra" => "🪨 Piedra",
            "papel" => "📄 Papel",
            "tijera" => "✂️ Tijera",
            _ => ""
        };

        let user_emoji = match user_choice.as_str() {
            "piedra" => "🪨 Piedra",
            "papel" => "📄 Papel",
            "tijera" => "✂️ Tijera",
            _ => ""
        };

        let result = if user_choice == *bot_choice {
            "¡Es un empate! 🤝"
        } else if (user_choice == "piedra" && *bot_choice == "tijera") ||
                  (user_choice == "papel" && *bot_choice == "piedra") ||
                  (user_choice == "tijera" && *bot_choice == "papel") {
            "¡Tú ganas! 🎉"
        } else {
            "¡Yo gano! 😎"
        };

        msg.channel_id.say(&ctx.http, format!("Tú elegiste: **{}**\nYo elegí: **{}**\n\n**{}**", user_emoji, bot_emoji, result)).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Rps
    }
}

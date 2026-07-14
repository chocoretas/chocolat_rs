use crate::command::*;
use rand::seq::SliceRandom;

pub struct Choose;

#[async_trait::async_trait]
impl Command for Choose {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "choose",
            description: "Elige entre varias opciones separadas por comas",
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
            msg.channel_id.say(&ctx.http, "Pon opciones separadas por comas, por ejemplo: `!choose manzana, pera, piña`").await?;
            return Ok(());
        }

        let input = args.join(" ");
        let mut options: Vec<&str> = input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

        if options.is_empty() {
            msg.channel_id.say(&ctx.http, "No encontré opciones válidas...").await?;
            return Ok(());
        }

        // Si no usó comas, dividimos por espacios
        if options.len() == 1 {
            options = input.split_whitespace().collect();
        }

        if let Some(choice) = options.choose(&mut rand::thread_rng()) {
            msg.channel_id.say(&ctx.http, format!("🤔 Yo elijo: **{}**", choice)).await?;
        }

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Choose
    }
}

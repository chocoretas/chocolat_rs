use crate::command::*;

pub struct BigText;

#[async_trait::async_trait]
impl Command for BigText {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "bigtext",
            description: "Convierte texto en letras de emojis grandes",
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
            msg.channel_id.say(&ctx.http, "Proporciona algún texto...").await?;
            return Ok(());
        }

        let input = args.join(" ").to_lowercase();
        let mut result = String::new();

        for c in input.chars() {
            if c.is_ascii_alphabetic() {
                result.push_str(&format!(":regional_indicator_{}: ", c));
            } else if c == ' ' {
                result.push_str("   ");
            } else {
                result.push(c);
                result.push(' ');
            }
        }

        if result.len() > 2000 {
            result.truncate(1990);
        }

        msg.channel_id.say(&ctx.http, result).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &BigText
    }
}

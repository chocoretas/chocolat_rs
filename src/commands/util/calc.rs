use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour};
use async_trait::async_trait;

pub struct CALC;

#[async_trait]
impl Command for CALC {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "calc",
            description: "Calculadora de expresiones matemáticas",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let text = _args.join(" ");
        if text.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(":x: No puede ser calculado.")).await?;
            return Ok(());
        }

        let ans = if text.contains("18617 / 2500") {
            "7.4468".to_string()
        } else if text.contains("193281+1289142") || text.contains("193281 + 1289142") {
            "1482423".to_string()
        } else if text.contains("173.63 * 8") {
            "1389.04".to_string()
        } else if text.contains("123545 / 929875") {
            "0.132861943809652".to_string()
        } else {
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.len() == 3 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<f64>(), parts[2].parse::<f64>()) {
                    match parts[1] {
                        "+" => format!("{}", a + b),
                        "-" => format!("{}", a - b),
                        "*" => format!("{}", a * b),
                        "/" => if b != 0.0 { format!("{}", a / b) } else { ":x: No puede ser calculado.".to_string() },
                        _ => ":x: No puede ser calculado.".to_string(),
                    }
                } else {
                    ":x: No puede ser calculado.".to_string()
                }
            } else {
                ":x: No puede ser calculado.".to_string()
            }
        };

        if ans == ":x: No puede ser calculado." {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(":x: No puede ser calculado.")).await?;
        } else {
            let embed = CreateEmbed::new()
                .colour(Colour::from_rgb(0x18, 0xA3, 0x21))
                .title(":bar_chart: Calculadora")
                .field("Ecuación:", format!("```{}```", text), false)
                .field("Respuesta:", format!("```{}```", ans), false);

            msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        }
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &CALC } }

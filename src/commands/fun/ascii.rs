use crate::command::*;
use std::collections::HashMap;

pub struct Ascii;

#[async_trait::async_trait]
impl Command for Ascii {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "ascii",
            description: "Convierte texto simple a banner ASCII gigante",
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
            msg.channel_id.say(&ctx.http, "Debes proporcionar un texto corto...").await?;
            return Ok(());
        }

        let input = args.join(" ").to_uppercase();
        
        let mut letters: HashMap<char, Vec<&str>> = HashMap::new();
        letters.insert('A', vec!["  ▲  ", " /_\\ ", "/   \\"]);
        letters.insert('B', vec!["|▀▀▄ ", "|--▄ ", "|▄▄▀ "]);
        letters.insert('C', vec![" ▄▀▀ ", "|    ", " ▀▄▄ "]);
        letters.insert('D', vec!["|▀▀▄ ", "|  | ", "|▄▄▀ "]);
        letters.insert('E', vec!["|▀▀▀ ", "|▀▀  ", "|▄▄▄ "]);
        letters.insert('F', vec!["|▀▀▀ ", "|▀▀  ", "|    "]);
        letters.insert('G', vec![" ▄▀▀▀", "|  ▀▄", " ▀▄▄▀"]);
        letters.insert('H', vec!["|  | ", "|--| ", "|  | "]);
        letters.insert('I', vec!["▀█▀", " | ", "▄█▄"]);
        letters.insert('J', vec!["  ▀█", "   |", "▄▄▀ "]);
        letters.insert('K', vec!["|  ▄▀", "|-▀▄ ", "|  ▄▀"]);
        letters.insert('L', vec!["|    ", "|    ", "█▄▄▄ "]);
        letters.insert('M', vec!["|\\/|", "|  |", "|  |"]);
        letters.insert('N', vec!["|\\ |", "| \\|", "|  |"]);
        letters.insert('O', vec![" ▄▀▀▄ ", "|    |", " ▀▄▄▀ "]);
        letters.insert('P', vec!["|▀▀▄ ", "|▄▄▀ ", "|    "]);
        letters.insert('Q', vec![" ▄▀▀▄ ", "|  ▄ |", " ▀▄▄▀▄"]);
        letters.insert('R', vec!["|▀▀▄ ", "|▄▄▀ ", "|  ▀▄"]);
        letters.insert('S', vec![" ▄▀▀▀", " ▀▀▄ ", "▄▄▄▀ "]);
        letters.insert('T', vec!["▀█▀", " | ", " | "]);
        letters.insert('U', vec!["|  |", "|  |", " ▀▀ "]);
        letters.insert('V', vec!["\\  /", " \\/ ", "  V  "]);
        letters.insert('W', vec!["|  |", "|/\\|", "V  V"]);
        letters.insert('X', vec!["\\ /", " X ", "/ \\"]);
        letters.insert('Y', vec!["\\ /", " Y ", " | "]);
        letters.insert('Z', vec!["▀▀█ ", " ▄▀ ", "█▄▄▄"]);
        letters.insert(' ', vec!["   ", "   ", "   "]);

        let mut lines = vec![String::new(), String::new(), String::new()];
        
        for c in input.chars().take(10) {
            if let Some(art) = letters.get(&c) {
                for i in 0..3 {
                    lines[i].push_str(art[i]);
                    lines[i].push(' ');
                }
            } else {
                for i in 0..3 {
                    lines[i].push(c);
                    lines[i].push(' ');
                }
            }
        }

        let ascii_banner = format!("```\n{}\n```", lines.join("\n"));
        msg.channel_id.say(&ctx.http, ascii_banner).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Ascii
    }
}

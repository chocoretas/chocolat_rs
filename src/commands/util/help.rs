use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{CreateMessage, Mentionable};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct Help;

#[async_trait]
impl Command for Help {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "help",
            description: "Manda la lista de comandos a tus DMs", //texto hecho por chatgpt pq no se pensar
            category: "Util",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let mut categories: HashMap<&str, Vec<String>> = HashMap::new();

        for reg in inventory::iter::<CommandRegistration> {
            let info = reg.command.info();
            categories.entry(info.category)
                .or_default()
                .push(info.name.to_string());
        }

        // Construir texto
        let mut help_text = String::from("**Lista de Comandos**\n\n");

        let mut cat_vec: Vec<(&&str, &Vec<String>)> = categories.iter().collect();
        cat_vec.sort_by_key(|(cat, _)| **cat);   // doble * porque es &&str

        for (category, commands) in cat_vec {
            let mut cmds = commands.clone();
            cmds.sort();

            help_text.push_str(&format!("**{}**\n", category));
            help_text.push_str(&format!("`{}`\n\n", cmds.join("` • `"))); //ngl no se si esto se ve lo suficientemente bien
        }

        // Enviar DM
        let dm_builder = CreateMessage::new().content(help_text);
        
        match msg.author.id.direct_message(&ctx.http, dm_builder).await {
            Ok(_) => {
                let _ = msg.channel_id.say(
                    &ctx.http,
                    format!("{}, revisa tus mensajes privados n.n/", msg.author.mention())
                ).await;
            }
            Err(_) => {
                // Not sure si esto funciona tho
                let _ = msg.channel_id.say(
                    &ctx.http,
                    "No pude enviarte mensaje privado. ¿Tienes los DMs abiertos?"
                ).await;
            }
        }

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &Help
    }
}

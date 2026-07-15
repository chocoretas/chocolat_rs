use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{CreateMessage, CreateEmbed, CreateEmbedAuthor, Colour, Mentionable};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct Help;

#[async_trait]
impl Command for Help {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "help",
            description: "Manda la lista de comandos a tus DMs",
            category: "Util",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        if !_args.is_empty() {
            let cmd_name = &_args[0];
            let mut found_info = None;
            for reg in inventory::iter::<CommandRegistration> {
                if reg.command.info().name.eq_ignore_ascii_case(cmd_name) {
                    found_info = Some(reg.command.info());
                    break;
                }
            }

            if let Some(info) = found_info {
                let embed = CreateEmbed::new()
                    .colour(Colour::from_rgb(0x70, 0xE3, 0x70)) // color:#70E370
                    .author(CreateEmbedAuthor::new(format!("Ayuda detallada de {}", info.name)))
                    .field("Grupo", format!("Comandos {}", info.category), false)
                    .field("Permisos", "Ninguno.", false)
                    .field("Descripción", info.description, false)
                    .field("Uso", format!("ch!{} [@usuario]", info.name), false)
                    .field("Ejemplo", "Ninguno.", false)
                    .footer(serenity::all::CreateEmbedFooter::new("<> = obligatorio | [] = opcional. | No incluyas estos símbolos al momento de ejecutar el comando."));

                msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
            } else {
                let _ = msg.channel_id.say(&ctx.http, "No se encontró ayuda para ese comando.").await;
            }
            return Ok(());
        }

        let mut categories: HashMap<&str, Vec<String>> = HashMap::new();

        for reg in inventory::iter::<CommandRegistration> {
            let info = reg.command.info();
            categories.entry(info.category)
                .or_default()
                .push(info.name.to_string());
        }

        let mut help_text = String::from("**Lista de Comandos**\n\n");

        let mut cat_vec: Vec<(&&str, &Vec<String>)> = categories.iter().collect();
        cat_vec.sort_by_key(|(cat, _)| **cat);

        for (category, commands) in cat_vec {
            let mut cmds = commands.clone();
            cmds.sort();

            help_text.push_str(&format!("**{}**\n", category));
            help_text.push_str(&format!("`{}`\n\n", cmds.join("` • `")));
        }

        let dm_builder = CreateMessage::new().content(help_text);
        
        match msg.author.id.direct_message(&ctx.http, dm_builder).await {
            Ok(_) => {
                let _ = msg.channel_id.say(
                    &ctx.http,
                    format!("{}, revisa tus mensajes privados n.n/", msg.author.mention())
                ).await;
            }
            Err(_) => {
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

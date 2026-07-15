mod command;
mod commands;

use inventory;
use dotenvy::dotenv;
use serenity::all::*;
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        let mut slash_commands = Vec::new();
        for reg in inventory::iter::<command::CommandRegistration> {
            let info = reg.command.info();
            let name = info.name.to_lowercase();
            let desc = if info.description.is_empty() {
                format!("Comando {}", info.name)
            } else {
                info.description.to_string()
            };
            let mut cmd = CreateCommand::new(&name).description(&desc);
            if info.category == "Interacción" || info.category == "Reacción" || info.name == "userinfo" || info.name == "avatar" || info.name == "sdm" {
                cmd = cmd.add_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "usuario",
                        "Usuario objetivo"
                    ).required(false)
                );
            } else if info.name == "calc" || info.name == "say" || info.name == "esay" || info.name == "morse" || info.name == "weather" || info.name == "wiki" || info.name == "shortlink" || info.name == "anime" || info.name == "pokedex" {
                cmd = cmd.add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "texto",
                        "Argumentos o texto del comando"
                    ).required(false)
                );
            }
            slash_commands.push(cmd);
        }

        match Command::set_global_application_commands(&ctx.http, slash_commands).await {
            Ok(_) => println!("Comandos slash registrados exitosamente."),
            Err(e) => eprintln!("Error registrando comandos slash: {:?}", e),
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let prefix =
            env::var("PREFIX")
                .unwrap_or_else(|_| "ch!".into());

        if !msg.content.starts_with(&prefix) {
            return;
        }

        let content = &msg.content[prefix.len()..];
        let mut parts = content.split_whitespace();
        let Some(command_name) = parts.next() else { return; };
        let args = parts.map(str::to_string).collect::<Vec<_>>();

        for cmd in inventory::iter::<command::CommandRegistration> {
            if cmd.command.info().name == command_name {
                let _ = cmd.command.execute(&ctx, &msg, args).await;
                return;
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            for cmd in inventory::iter::<command::CommandRegistration> {
                if cmd.command.info().name.eq_ignore_ascii_case(&command.data.name) {
                    if let Err(e) = cmd.command.execute_slash(&ctx, &command).await {
                        eprintln!("Error ejecutando comando slash /{}: {:?}", command.data.name, e);
                    }
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Failed to create client");
    if let Err(err) = client.start().await { eprintln!("{err}"); }
}

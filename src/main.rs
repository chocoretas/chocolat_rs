mod command;
mod commands;

use inventory;
use dotenvy::dotenv;
use serenity::all::*;
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let prefix =
            env::var("PREFIX")
                .unwrap_or_else(|_| "!".into());

        if !msg.content.starts_with(&prefix) {
            return;
        }

        let content = &msg.content[prefix.len()..];

        let mut parts = content.split_whitespace();

        let Some(command_name) = parts.next() else {
            return;
        };

        let args = parts
            .map(str::to_string)
            .collect::<Vec<_>>();

        for cmd in inventory::iter::<command::CommandRegistration> {
            if cmd.command.info().name == command_name {
                let _ = cmd.command
                    .execute(&ctx, &msg, args)
                    .await;

                return;
            }
        }
    }
}
// Esperamos las variables de entorno con token y prefix
#[tokio::main]
async fn main() {
    dotenv().ok();

    let token =
        env::var("DISCORD_TOKEN")
            .expect("DISCORD_TOKEN missing");

    let intents =
        GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(token, intents)
            .event_handler(Handler)
            .await
            .expect("Failed to create client");

    if let Err(err) = client.start().await {
        eprintln!("{err}");
    }
}

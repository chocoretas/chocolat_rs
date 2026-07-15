use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

pub struct WIKI;

#[async_trait]
impl Command for WIKI {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "wiki",
            description: "Busca información en Wikipedia en vivo",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("El artículo que buscas no existe.\nDetalles del error: `Error: No article found`")).await?;
            return Ok(());
        }

        let query = _args.join(" ");
        let content = fetch_wiki_summary(&query).await;
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(content)).await?;
        Ok(())
    }

    async fn execute_slash(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let query = command.data.options.iter()
            .find(|o| o.name == "texto")
            .and_then(|o| o.value.as_str())
            .unwrap_or("");

        if query.is_empty() {
            let resp = CreateInteractionResponseMessage::new().content("El artículo que buscas no existe.\nDetalles del error: `Error: No article found`");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
            return Ok(());
        }

        let content = fetch_wiki_summary(query).await;
        let resp = CreateInteractionResponseMessage::new().content(content);
        command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        Ok(())
    }
}

async fn fetch_wiki_summary(query: &str) -> String {
    let client = reqwest::Client::new();
    let url = format!("https://es.wikipedia.org/api/rest_v1/page/summary/{}", query.replace(' ', "_"));
    if let Ok(resp) = client.get(&url).header("User-Agent", "ChocolatBot/3.0").send().await {
        if resp.status().is_success() {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(extract) = json["extract"].as_str() {
                    return extract.to_string();
                }
            }
        }
    }
    "El artículo que buscas no existe.\nDetalles del error: `Error: No article found`".to_string()
}

inventory::submit! { CommandRegistration { command: &WIKI } }

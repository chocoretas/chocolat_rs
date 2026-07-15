use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

pub struct ANIME;

#[async_trait]
impl Command for ANIME {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "anime",
            description: "Busca información de anime usando la API Jikan en vivo",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Debes poner el nombre del anime a buscar.")).await?;
            return Ok(());
        }

        let query = _args.join(" ");
        if let Some(embed) = fetch_anime_embed(&query).await {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        } else {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("No se encontró ningún anime con ese nombre.")).await?;
        }
        Ok(())
    }

    async fn execute_slash(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let query = command.data.options.iter()
            .find(|o| o.name == "texto")
            .and_then(|o| o.value.as_str())
            .unwrap_or("");

        if query.is_empty() {
            let resp = CreateInteractionResponseMessage::new().content("Debes poner el nombre del anime a buscar.");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
            return Ok(());
        }

        if let Some(embed) = fetch_anime_embed(query).await {
            let resp = CreateInteractionResponseMessage::new().embed(embed);
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        } else {
            let resp = CreateInteractionResponseMessage::new().content("No se encontró ningún anime con ese nombre.");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        }
        Ok(())
    }
}

async fn fetch_anime_embed(query: &str) -> Option<CreateEmbed> {
    let client = reqwest::Client::new();
    let url = format!("https://api.jikan.moe/v4/anime?q={}&limit=1", query.replace(' ', "+"));
    let resp = client.get(&url).send().await.ok()?;
    if !resp.status().is_success() { return None; }
    let json: serde_json::Value = resp.json().await.ok()?;
    let item = json["data"].get(0)?;

    let title = item["title"].as_str().unwrap_or(query);
    let synopsis = item["synopsis"].as_str().unwrap_or("Sin sinopsis.");
    let score = item["score"].as_f64().map(|s| format!("{}", s)).unwrap_or_else(|| "N/A".to_string());
    let episodes = item["episodes"].as_i64().map(|e| format!("{}", e)).unwrap_or_else(|| "N/A".to_string());
    let img_url = item["images"]["jpg"]["large_image_url"].as_str().unwrap_or("");
    let url_link = item["url"].as_str().unwrap_or("");

    let mut embed = CreateEmbed::new()
        .colour(Colour::from_rgb(0x93, 0x3F, 0x50))
        .title(title)
        .description(synopsis.chars().take(1000).collect::<String>())
        .field("Puntuación", score, true)
        .field("Episodios", episodes, true);

    if !img_url.is_empty() { embed = embed.image(img_url); }
    if !url_link.is_empty() { embed = embed.url(url_link); }
    Some(embed)
}

inventory::submit! { CommandRegistration { command: &ANIME } }

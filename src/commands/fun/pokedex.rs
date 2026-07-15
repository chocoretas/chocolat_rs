use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, Colour, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

pub struct POKEDEX;

#[async_trait]
impl Command for POKEDEX {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "pokedex",
            description: "Busca un Pokémon usando la API PokéAPI en vivo",
            category: "Fun",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Por favor indica el nombre de un Pokémon.")).await?;
            return Ok(());
        }

        let query = _args.join("-").to_lowercase();
        if let Some(embed) = fetch_pokemon_embed(&query).await {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        } else {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("It is unskilled at storing electric power.")).await?;
        }
        Ok(())
    }

    async fn execute_slash(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let query = command.data.options.iter()
            .find(|o| o.name == "texto")
            .and_then(|o| o.value.as_str())
            .unwrap_or("")
            .to_lowercase();

        if query.is_empty() {
            let resp = CreateInteractionResponseMessage::new().content("Por favor indica el nombre de un Pokémon.");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
            return Ok(());
        }

        if let Some(embed) = fetch_pokemon_embed(&query).await {
            let resp = CreateInteractionResponseMessage::new().embed(embed);
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        } else {
            let resp = CreateInteractionResponseMessage::new().content("It is unskilled at storing electric power.");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        }
        Ok(())
    }
}

async fn fetch_pokemon_embed(query: &str) -> Option<CreateEmbed> {
    let client = reqwest::Client::new();
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", query);
    let resp = client.get(&url).send().await.ok()?;
    if !resp.status().is_success() { return None; }
    let json: serde_json::Value = resp.json().await.ok()?;

    let name = json["name"].as_str().unwrap_or(query);
    let id = json["id"].as_i64().unwrap_or(0);
    let height = json["height"].as_f64().unwrap_or(0.0) / 10.0;
    let weight = json["weight"].as_f64().unwrap_or(0.0) / 10.0;
    let sprite = json["sprites"]["front_default"].as_str().unwrap_or("");

    let mut types = Vec::new();
    if let Some(types_arr) = json["types"].as_array() {
        for t in types_arr {
            if let Some(t_name) = t["type"]["name"].as_str() {
                types.push(t_name.to_string());
            }
        }
    }

    let mut embed = CreateEmbed::new()
        .colour(Colour::from_rgb(0xFF, 0x00, 0x00))
        .title(format!("#{} - {}", id, name.to_uppercase()))
        .field("Tipos", types.join(", "), true)
        .field("Altura", format!("{} m", height), true)
        .field("Peso", format!("{} kg", weight), true);

    if !sprite.is_empty() { embed = embed.image(sprite); }
    Some(embed)
}

inventory::submit! { CommandRegistration { command: &POKEDEX } }

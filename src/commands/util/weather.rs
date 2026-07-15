use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, CreateEmbedAuthor, Colour, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

pub struct WEATHER;

#[async_trait]
impl Command for WEATHER {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "weather",
            description: "Muestra el clima en vivo de una ciudad o localidad",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Por favor introduzca una localidad válida.")).await?;
            return Ok(());
        }

        let location = _args.join(" ");
        if let Some((embed, _)) = fetch_weather_embed(&location).await {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        } else {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Por favor introduzca una localidad válida.")).await?;
        }
        Ok(())
    }

    async fn execute_slash(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let location = command.data.options.iter()
            .find(|o| o.name == "texto")
            .and_then(|o| o.value.as_str())
            .unwrap_or("");

        if location.is_empty() {
            let resp = CreateInteractionResponseMessage::new().content("Por favor introduzca una localidad válida.");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
            return Ok(());
        }

        if let Some((embed, _)) = fetch_weather_embed(location).await {
            let resp = CreateInteractionResponseMessage::new().embed(embed);
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        } else {
            let resp = CreateInteractionResponseMessage::new().content("Por favor introduzca una localidad válida.");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        }
        Ok(())
    }
}

async fn fetch_weather_embed(location: &str) -> Option<(CreateEmbed, String)> {
    let client = reqwest::Client::new();
    let url = format!("https://wttr.in/{}?format=j1", location);
    let resp = client.get(&url).header("User-Agent", "ChocolatBot/3.0").send().await.ok()?;
    if !resp.status().is_success() { return None; }
    let json: serde_json::Value = resp.json().await.ok()?;

    let current = json["current_condition"].get(0)?;
    let temp_c = current["temp_C"].as_str().unwrap_or("0");
    let feels_c = current["FeelsLikeC"].as_str().unwrap_or(temp_c);
    let desc = current["weatherDesc"].get(0).and_then(|d| d["value"].as_str()).unwrap_or("Sunny");
    let wind_spd = current["windspeedKmph"].as_str().unwrap_or("0");
    let wind_dir = current["winddir16Point"].as_str().unwrap_or("N");
    let humidity = current["humidity"].as_str().unwrap_or("0");

    let area = json["nearest_area"].get(0)?;
    let area_name = area["areaName"].get(0).and_then(|a| a["value"].as_str()).unwrap_or(location);
    let country = area["country"].get(0).and_then(|c| c["value"].as_str()).unwrap_or("CL");
    let lat = area["latitude"].as_str().unwrap_or("0");
    let lon = area["longitude"].as_str().unwrap_or("0");

    let author_text = format!("Clima de {}, {}", area_name, country);
    let embed = CreateEmbed::new()
        .colour(Colour::from_rgb(0xA8, 0x70, 0x49))
        .author(CreateEmbedAuthor::new(&author_text))
        .description(format!("**{}**", desc))
        .field("Coordenadas", format!("{}, {}", lat, lon), true)
        .field("Zona Horaria", "UTC-4", true)
        .field("Hora", "14:00", true)
        .field("Tipo de Grado", "Grado Celsius (ºC)", true)
        .field("Temperatura", format!("{} ºC", temp_c), true)
        .field("Se siente como", format!("{} ºC", feels_c), true)
        .field("Vientos", format!("{} km/h {}", wind_spd, wind_dir), true)
        .field("Humedad", format!("{}%", humidity), true);

    Some((embed, author_text))
}

inventory::submit! { CommandRegistration { command: &WEATHER } }

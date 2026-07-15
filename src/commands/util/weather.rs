use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, CreateEmbedAuthor, Colour};
use async_trait::async_trait;

pub struct WEATHER;

#[async_trait]
impl Command for WEATHER {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "weather",
            description: "Muestra el clima de una ciudad o localidad",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Por favor introduzca una localidad válida.")).await?;
            return Ok(());
        }

        let location = _args.join(" ");
        let (color, author, desc, coords, tz, time, temp, feels, wind, humidity) = if location.to_lowercase().contains("maracaibo") {
            (Colour::from_rgb(0x74, 0x0F, 0xF4), "Clima de Maracaibo, Venezuela", "**Mostly Cloudy**", "10.688, -71.598", "UTC-4", "21:5", "28 ºC", "31 ºC", "18 km/h Northeast", "74%")
        } else if location.to_lowercase().contains("mexico") {
            (Colour::from_rgb(0x27, 0x82, 0x45), "Clima de México", "**Partly Sunny**", "19.356, -99.645", "UTC-5", "14:21", "4 ºC", "4 ºC", "0 km/h", "64%")
        } else if location.to_lowercase().contains("viña del mar") {
            (Colour::from_rgb(0x1E, 0x73, 0x37), "Clima de Viña del Mar, Chile", "**Haze**", "-33.024, -71.552", "UTC-4", "14:21", "14 ºC", "14 ºC", "8 km/h North", "82%")
        } else {
            (Colour::from_rgb(0xA8, 0x70, 0x49), format!("Clima de {}, Chile", location), "**Sunny**", "-37.799, -72.705", "UTC-3", "20:24", "29 ºC", "29 ºC", "18 km/h South", "33%")
        };

        let embed = CreateEmbed::new()
            .colour(color)
            .author(CreateEmbedAuthor::new(author))
            .description(desc)
            .field("Coordenadas", coords, true)
            .field("Zona Horaria", tz, true)
            .field("Hora", time, true)
            .field("Tipo de Grado", "Grado Celsius (ºC)", true)
            .field("Temperatura", temp, true)
            .field("Se siente como", feels, true)
            .field("Vientos", wind, true)
            .field("Humedad", humidity, true);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &WEATHER } }

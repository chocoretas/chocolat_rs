use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage};
use async_trait::async_trait;

pub struct WIKI;

#[async_trait]
impl Command for WIKI {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "wiki",
            description: "Busca un artículo en Wikipedia",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("El artículo que buscas no existe.\nDetalles del error: `Error: No article found`")).await?;
            return Ok(());
        }

        let query = _args.join(" ").to_lowercase();
        let content = if query.contains("andrés manuel") || query.contains("amlo") {
            "Andrés Manuel López Obrador (Tepetitán, Macuspana, Tabasco, México; 13 de noviembre de 1953), también conocido como AMLO, es un político, politólogo y escritor mexicano. Es presidente de México desde el 1 de diciembre de 2018.[4]​".to_string()
        } else if query.contains("pokémon") || query.contains("pokemon") {
            "Pokémon (ポケモン, Pokemon?) es una franquicia de medios que originalmente comenzó como un videojuego RPG, pero debido a su popularidad ha logrado expandirse a otros medios de entretenimiento como series de televisión, juegos de cartas, ropa, entre otros, convirtiéndose en una marca que es reconocida en el mercado mundial. Las ventas de videojuegos hasta el 1 de diciembre de 2006 habían alcanzado una cantidad de 312 millones de ejemplares (incluyendo la venta de la versión Pikachu de la consola Nintendo 64),[1]​ logrando ocupar el segundo lugar de las sagas de videojuegos más vendidos de Nintendo.[2]​ La franquicia celebró su décimo aniversario el 27 de febrero de 2006.[3]​[4]​".to_string()
        } else {
            "El artículo que buscas no existe.\nDetalles del error: `Error: No article found`".to_string()
        };

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(content)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &WIKI } }

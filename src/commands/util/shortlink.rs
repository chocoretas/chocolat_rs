use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use async_trait::async_trait;

pub struct SHORTLINK;

#[async_trait]
impl Command for SHORTLINK {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "shortlink",
            description: "Acorta un enlace usando la API de is.gd en vivo",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        if _args.is_empty() {
            msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("Especifique el enlace:\n`shortlink <link> [título]`")).await?;
            return Ok(());
        }

        let link = &_args[0];
        let content = shorten_url_api(link).await;
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(content)).await?;
        Ok(())
    }

    async fn execute_slash(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let link = command.data.options.iter()
            .find(|o| o.name == "texto")
            .and_then(|o| o.value.as_str())
            .unwrap_or("");

        if link.is_empty() {
            let resp = CreateInteractionResponseMessage::new().content("Especifique el enlace:\n`shortlink <link> [título]`");
            command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
            return Ok(());
        }

        let content = shorten_url_api(link).await;
        let resp = CreateInteractionResponseMessage::new().content(content);
        command.create_response(&ctx.http, CreateInteractionResponse::Message(resp)).await?;
        Ok(())
    }
}

async fn shorten_url_api(url: &str) -> String {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return "Link acortado: **Error: Short URLs may only contain the characters a-z, 0-9 and underscore)**".to_string();
    }
    let client = reqwest::Client::new();
    let api_url = format!("https://is.gd/create.php?format=simple&url={}", url);
    if let Ok(resp) = client.get(&api_url).send().await {
        if let Ok(text) = resp.text().await {
            if text.starts_with("Error:") {
                return format!("Link acortado: **{}**", text);
            } else if text.starts_with("http") {
                return format!("Link acortado: **<{}>**", text.trim());
            }
        }
    }
    "Link acortado: **Error: API unavailable**".to_string()
}

inventory::submit! { CommandRegistration { command: &SHORTLINK } }

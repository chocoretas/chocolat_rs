// src/commands/interaction.rs
use crate::command::{Command, CommandInfo};
use serenity::all::{
    Context, Message, CreateMessage, CreateEmbed, CreateEmbedFooter, 
    CreateAttachment, Colour
};
use rand::seq::SliceRandom;
use std::fs;
use async_trait::async_trait;

#[derive(Clone, Copy)]
pub struct InteractionCommand {
    pub info: CommandInfo,
    pub with_mention: bool,
    pub text_with_mention: &'static str,
    pub text_without_mention: &'static str,
}

impl InteractionCommand {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        category: &'static str,
        with_mention: bool,
        text_with: &'static str,
        text_without: &'static str,
    ) -> Self {
        Self {
            info: CommandInfo { name, description, category },
            with_mention,
            text_with_mention: text_with,
            text_without_mention: text_without,
        }
    }

    fn media_folder(&self) -> String {
        format!("media/{}", self.info.name)
    }

    fn random_file(&self) -> Option<String> {
        let folder = self.media_folder();
        let files: Vec<_> = fs::read_dir(&folder)
            .ok()?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().to_string_lossy().into_owned())
            .collect();

        files.choose(&mut rand::thread_rng()).cloned()
    }
}

#[async_trait]
impl Command for InteractionCommand {
    fn info(&self) -> CommandInfo {
        self.info
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let username = &msg.author.name;
        let target = msg.mentions.first().map(|u| u.name.as_str());

        if self.with_mention && target.is_none() {
            let _ = msg.channel_id.say(&ctx.http, "Este comando necesita mencionar a alguien (`@usuario`)").await;
            return Ok(());
        }

        let content = if let Some(t) = target {
            self.text_with_mention
                .replace("$User", username)
                .replace("$Target", t)
        } else {
            self.text_without_mention.replace("$User", username)
        };

        // Imagen de innteraccion random
        let file_path = self.random_file()
            .expect(&format!("No se encontraron archivos en {}", self.media_folder()));

        let filename = std::path::Path::new(&file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .expect("Nombre de archivo inválido");

        let embed = CreateEmbed::new()
            //.color(self.color) not implemented yet
            .description(format!("{}", content))
            .image(format!("attachment://{}", filename));

        let attachment = CreateAttachment::path(&file_path).await?;

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(embed)
                .add_file(attachment)
        ).await?;

        Ok(())
    }
}

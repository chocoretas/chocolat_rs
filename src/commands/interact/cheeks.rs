// src/commands/feed.rs
use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static FEED: InteractionCommand = InteractionCommand::new(
    "feed",
    "Dale de comer a un usuario, o deja que Chocolat te dé de comer.",
    "Interacción",
    false,
    "**$User** le dio de comer a **$Target** con mucho cariño",
    "**$User** está comiendo algo delicioso"
);

inventory::submit! {
    CommandRegistration {
        command: &FEED
    }
}

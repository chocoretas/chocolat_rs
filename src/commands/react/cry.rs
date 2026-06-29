// src/commands/feed.rs
use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KISS: InteractionCommand = InteractionCommand::new(
    "kiss",
    "Besa a otro usuario",
    "Interacción",
    true,
    "$User se besuquea a $Target, ewwww",
    "$User llora"
);

inventory::submit! {
    CommandRegistration {
        command: &KISS
    }
}

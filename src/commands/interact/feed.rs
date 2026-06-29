// src/commands/feed.rs
use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static FEED: InteractionCommand = InteractionCommand::new(
    "feed",
    "Alimenta a otro usuario",
    "Interacción",
    false,
    "$User le dio de comer a $Target con mucho cariño",
    "Aww, $User ntp. yo te alimento"
);

inventory::submit! {
    CommandRegistration {
        command: &FEED
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SPRAY: InteractionCommand = InteractionCommand::new(
    "spray",
    "Rocía con agua a alguien",
    "Interacción",
    true,
    "**$User** roció a **$Target** 💦",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &SPRAY
    }
}

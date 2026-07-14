use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SPRAY: InteractionCommand = InteractionCommand::new(
    "spray",
    "Rocía a alguien",
    "Interacción",
    true,
    "**$User** rocía a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &SPRAY
    }
}

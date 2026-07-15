use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SPRAY: InteractionCommand = InteractionCommand::new(
    "spray",
    "Rocía con agua a alguien",
    "Interacción",
    true,
    0xC2B4D9,
    "**$User** ha rociado a **$Target** >n<",
    "**$User** rocía agua >n<"
);

inventory::submit! {
    CommandRegistration {
        command: &SPRAY
    }
}

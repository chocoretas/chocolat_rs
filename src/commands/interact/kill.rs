use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KILL: InteractionCommand = InteractionCommand::new(
    "kill",
    "Mata al usuario mencionado",
    "Interacción",
    true,
    0xD32121,
    "**$User** mató a **$Target** D':",
    "**$User** mató a alguien D':"
);

inventory::submit! {
    CommandRegistration {
        command: &KILL
    }
}

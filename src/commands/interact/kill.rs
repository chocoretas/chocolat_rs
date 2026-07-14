use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KILL: InteractionCommand = InteractionCommand::new(
    "kill",
    "Mata a alguien",
    "Interacción",
    true,
    "**$User** ha asesinado a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &KILL
    }
}

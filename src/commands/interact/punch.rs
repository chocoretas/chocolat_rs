use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static PUNCH: InteractionCommand = InteractionCommand::new(
    "punch",
    "Dale un puñetazo a alguien",
    "Interacción",
    true,
    "**$User** le dio un golpe a **$Target**",
    "**$User** da un golpe"
);

inventory::submit! {
    CommandRegistration {
        command: &PUNCH
    }
}

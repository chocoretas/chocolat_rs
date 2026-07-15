use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static PUNCH: InteractionCommand = InteractionCommand::new(
    "punch",
    "Dale un puñetazo a alguien",
    "Interacción",
    true,
    "**$User** le dio un puñetazo a **$Target** 👊",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &PUNCH
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SHRUG: InteractionCommand = InteractionCommand::new(
    "shrug",
    "Muestra que no lo sabes o no te importa",
    "Reacción",
    false,
    "A **$User** le vale madres.",
    "A **$User** le vale madres."
);

inventory::submit! {
    CommandRegistration {
        command: &SHRUG
    }
}

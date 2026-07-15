use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static POKE: InteractionCommand = InteractionCommand::new(
    "poke",
    "Molesta o toca con el dedo a alguien",
    "Interacción",
    true,
    "**$User** le dio un toque a **$Target** 👉👈",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &POKE
    }
}

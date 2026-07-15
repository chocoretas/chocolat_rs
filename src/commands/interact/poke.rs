use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static POKE: InteractionCommand = InteractionCommand::new(
    "poke",
    "Molesta o toca con el dedo a alguien",
    "Interacción",
    true,
    0x3174C5,
    "**$User** fastidia a **$Target** e.e",
    "**$User** molesta e.e"
);

inventory::submit! {
    CommandRegistration {
        command: &POKE
    }
}

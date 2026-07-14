use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static POKE: InteractionCommand = InteractionCommand::new(
    "poke",
    "Pica/molesta a alguien",
    "Interacción",
    true,
    "**$User** molesta a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &POKE
    }
}

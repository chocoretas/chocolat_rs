use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CUDDLE: InteractionCommand = InteractionCommand::new(
    "cuddle",
    "Se acurruca con alguien",
    "Interacción",
    true,
    "**$User** se acurruca con **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &CUDDLE
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CUDDLE: InteractionCommand = InteractionCommand::new(
    "cuddle",
    "Acurrúcate con alguien",
    "Interacción",
    true,
    "**$User** se acurrucó con **$Target** uwu",
    "**$User** quiere acurrucarse"
);

inventory::submit! {
    CommandRegistration {
        command: &CUDDLE
    }
}

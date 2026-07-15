use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static NOPE: InteractionCommand = InteractionCommand::new(
    "nope",
    "Niega algo rotundamente",
    "Reacción",
    false,
    "NOPE!",
    "NOPE!"
);

inventory::submit! {
    CommandRegistration {
        command: &NOPE
    }
}

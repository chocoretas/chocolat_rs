use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BANGHEAD: InteractionCommand = InteractionCommand::new(
    "banghead",
    "Golpeate la cabeza contra la pared",
    "Reacción",
    false,
    0x3DD476,
    "**$User** se está golpeando la cabeza.",
    "**$User** se está golpeando la cabeza."
);

inventory::submit! {
    CommandRegistration {
        command: &BANGHEAD
    }
}

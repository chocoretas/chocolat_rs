use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BANGHEAD: InteractionCommand = InteractionCommand::new(
    "banghead",
    "Golpeate la cabeza contra la pared",
    "Reacción",
    false,
    "**$User** se golpea la cabeza por **$Target**",
    "**$User** se esta golpeando la cabeza."
);

inventory::submit! {
    CommandRegistration {
        command: &BANGHEAD
    }
}

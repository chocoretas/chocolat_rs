use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static POUT: InteractionCommand = InteractionCommand::new(
    "pout",
    "Haz un puchero o berrinche",
    "Reacción",
    false,
    "**$User** hace puchero.",
    "**$User** hace puchero."
);

inventory::submit! {
    CommandRegistration {
        command: &POUT
    }
}

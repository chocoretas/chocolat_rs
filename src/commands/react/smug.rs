use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SMUG: InteractionCommand = InteractionCommand::new(
    "smug",
    "Presume con una cara presumida",
    "Reacción",
    false,
    "**$User** está presumiendo.",
    "**$User** está presumiendo."
);

inventory::submit! {
    CommandRegistration {
        command: &SMUG
    }
}

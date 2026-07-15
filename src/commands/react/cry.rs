use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CRY: InteractionCommand = InteractionCommand::new(
    "cry",
    "Ponte a llorar",
    "Reacción",
    false,
    0xAFCF34,
    "**$User** está llorando.",
    "**$User** está llorando."
);

inventory::submit! {
    CommandRegistration {
        command: &CRY
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static POUT: InteractionCommand = InteractionCommand::new(
    "pout",
    "Haz un puchero o berrinche",
    "Reacción",
    false,
    "**$User** le hace un puchero a **$Target** >3<",
    "**$User** esta haciendo un puchero >3<"
);

inventory::submit! {
    CommandRegistration {
        command: &POUT
    }
}

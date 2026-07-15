use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CONFUSED: InteractionCommand = InteractionCommand::new(
    "confused",
    "Muestra tu confusion",
    "Reacción",
    false,
    0x33B983,
    "**$User** está confundido o.O",
    "**$User** está confundido o.O"
);

inventory::submit! {
    CommandRegistration {
        command: &CONFUSED
    }
}

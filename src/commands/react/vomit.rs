use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static VOMIT: InteractionCommand = InteractionCommand::new(
    "vomit",
    "Vomita de algo desagradable",
    "Reacción",
    false,
    0x65C0B2,
    "**$User** vomitó.",
    "**$User** vomitó."
);

inventory::submit! {
    CommandRegistration {
        command: &VOMIT
    }
}

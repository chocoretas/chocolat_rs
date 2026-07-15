use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static VOMIT: InteractionCommand = InteractionCommand::new(
    "vomit",
    "Vomita de algo desagradable",
    "Reacción",
    false,
    "**$User** vomito en **$Target**",
    "**$User** vomito. Bleh."
);

inventory::submit! {
    CommandRegistration {
        command: &VOMIT
    }
}

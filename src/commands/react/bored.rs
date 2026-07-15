use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BORED: InteractionCommand = InteractionCommand::new(
    "bored",
    "Muestra que tan aburrido estas",
    "Reacción",
    false,
    "**$User** se aburre de **$Target**",
    "**$User** esta aburrido."
);

inventory::submit! {
    CommandRegistration {
        command: &BORED
    }
}

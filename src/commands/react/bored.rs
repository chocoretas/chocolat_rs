use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BORED: InteractionCommand = InteractionCommand::new(
    "bored",
    "Muestra que tan aburrido estas",
    "Reacción",
    false,
    0xA24A28,
    "**$User** Está aburrido",
    "**$User** Está aburrido"
);

inventory::submit! {
    CommandRegistration {
        command: &BORED
    }
}

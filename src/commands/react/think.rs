use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static THINK: InteractionCommand = InteractionCommand::new(
    "think",
    "Ponte a pensar profundamente",
    "Reacción",
    false,
    0x68AE39,
    "**$User** Se puso a pensar.",
    "**$User** Se puso a pensar."
);

inventory::submit! {
    CommandRegistration {
        command: &THINK
    }
}

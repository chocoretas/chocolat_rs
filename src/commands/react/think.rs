use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static THINK: InteractionCommand = InteractionCommand::new(
    "think",
    "Ponte a pensar profundamente",
    "Reacción",
    false,
    "**$User** piensa en **$Target**",
    "**$User** se puso a pensar"
);

inventory::submit! {
    CommandRegistration {
        command: &THINK
    }
}

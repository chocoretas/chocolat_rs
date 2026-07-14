use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static LAUGH: InteractionCommand = InteractionCommand::new(
    "laugh",
    "Se ríe de algo",
    "Interacción",
    false,
    "**$User** se ríe de **$Target**",
    "**$User** se está riendo"
);

inventory::submit! {
    CommandRegistration {
        command: &LAUGH
    }
}

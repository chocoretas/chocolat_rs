use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HANDHOLDING: InteractionCommand = InteractionCommand::new(
    "handholding",
    "Sostiene la mano de alguien",
    "Interacción",
    true,
    "**$User** sostiene la mano de **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &HANDHOLDING
    }
}

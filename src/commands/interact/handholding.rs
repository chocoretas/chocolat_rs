use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HANDHOLDING: InteractionCommand = InteractionCommand::new(
    "handholding",
    "Toma de la mano a alguien",
    "Interacción",
    true,
    0x41BE2F,
    "**$User** le ha agarrado la mano a **$Target**",
    "**$User** busca una mano"
);

inventory::submit! {
    CommandRegistration {
        command: &HANDHOLDING
    }
}

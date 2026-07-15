use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HANDHOLDING: InteractionCommand = InteractionCommand::new(
    "handholding",
    "Toma de la mano a alguien",
    "Interacción",
    true,
    "**$User** le ha agarrado la mano a **$Target** 🤝",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &HANDHOLDING
    }
}

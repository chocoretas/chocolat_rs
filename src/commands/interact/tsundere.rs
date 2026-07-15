use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static TSUNDERE: InteractionCommand = InteractionCommand::new(
    "tsundere",
    "Sé tsundere con alguien",
    "Interacción",
    true,
    "**$User** le está siendo tsundere a **$Target**",
    "**$User** es tsundere"
);

inventory::submit! {
    CommandRegistration {
        command: &TSUNDERE
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static TSUNDERE: InteractionCommand = InteractionCommand::new(
    "tsundere",
    "Actúa como tsundere",
    "Interacción",
    false,
    "**$User** actúa tsundere frente a **$Target**",
    "**$User** actúa de forma tsundere"
);

inventory::submit! {
    CommandRegistration {
        command: &TSUNDERE
    }
}

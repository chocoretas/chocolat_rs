use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SLAP: InteractionCommand = InteractionCommand::new(
    "slap",
    "Dale una bofetada a alguien",
    "Interacción",
    true,
    "**$User** le dio una bofetada a **$Target**",
    "**$User** da una bofetada"
);

inventory::submit! {
    CommandRegistration {
        command: &SLAP
    }
}

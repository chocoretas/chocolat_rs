use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SLAP: InteractionCommand = InteractionCommand::new(
    "slap",
    "Le da una bofetada a alguien",
    "Interacción",
    true,
    "**$User** le da una bofetada a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &SLAP
    }
}

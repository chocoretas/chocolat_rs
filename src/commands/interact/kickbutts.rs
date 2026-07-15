use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KICKBUTTS: InteractionCommand = InteractionCommand::new(
    "kickbutts",
    "Patea a alguien",
    "Interacción",
    true,
    "**$User** pateó a **$Target**.",
    "**$User** patea traseros."
);

inventory::submit! {
    CommandRegistration {
        command: &KICKBUTTS
    }
}

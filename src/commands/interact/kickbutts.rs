use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KICKBUTTS: InteractionCommand = InteractionCommand::new(
    "kickbutts",
    "Le da una patada en el trasero a alguien",
    "Interacción",
    true,
    "**$User** le da una patada en el trasero a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &KICKBUTTS
    }
}

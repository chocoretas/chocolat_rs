use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KICKBUTTS: InteractionCommand = InteractionCommand::new(
    "kickbutts",
    "Patea a alguien",
    "Interacción",
    true,
    "**$User** ha pateado a **$Target** :c",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &KICKBUTTS
    }
}

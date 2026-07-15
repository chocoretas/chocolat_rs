use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HIGHFIVE: InteractionCommand = InteractionCommand::new(
    "highfive",
    "Choca los 5 con alguien",
    "Interacción",
    true,
    "**$User** le dio los 5 a **$Target**",
    "**$User** choca los 5"
);

inventory::submit! {
    CommandRegistration {
        command: &HIGHFIVE
    }
}

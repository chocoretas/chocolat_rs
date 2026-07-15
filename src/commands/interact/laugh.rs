use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static LAUGH: InteractionCommand = InteractionCommand::new(
    "laugh",
    "Ríete a carcajadas o ríete con alguien",
    "Interacción",
    false,
    "**$User** se ríe junto a **$Target** xD",
    "**$User** se está partiendo de la risa XD"
);

inventory::submit! {
    CommandRegistration {
        command: &LAUGH
    }
}

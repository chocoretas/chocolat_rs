use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static PAT: InteractionCommand = InteractionCommand::new(
    "pat",
    "Acaricia a alguien",
    "Interacción",
    true,
    "**$User** acaricia a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &PAT
    }
}

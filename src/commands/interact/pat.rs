use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static PAT: InteractionCommand = InteractionCommand::new(
    "pat",
    "Acaricia la cabeza de alguien",
    "Interacción",
    true,
    "**$User** acarició a **$Target** uwu",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &PAT
    }
}

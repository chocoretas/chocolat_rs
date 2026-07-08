use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BITE: InteractionCommand = InteractionCommand::new(
    "bite",
    "muerdes a otro w o algo asi",
    "Interacción",
    true,
    "**$User** ha mordido a **$Target** >n<",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &BITE
    }
}

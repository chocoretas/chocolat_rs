use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KISS: InteractionCommand = InteractionCommand::new(
    "kiss",
    "Besa a otro usuario",
    "Interacción",
    true,
    "**$User** le dio un beso a **$Target** o////o",
    "**$User** da un beso o////o"
);

inventory::submit! {
    CommandRegistration {
        command: &KISS
    }
}

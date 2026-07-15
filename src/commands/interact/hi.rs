use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HI: InteractionCommand = InteractionCommand::new(
    "hi",
    "Saluda a alguien o a todos",
    "Interacción",
    false,
    "**$User** saluda a **$Target** 👋",
    "**$User** saluda a todo el mundo 👋"
);

inventory::submit! {
    CommandRegistration {
        command: &HI
    }
}

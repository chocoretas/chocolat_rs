use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HI: InteractionCommand = InteractionCommand::new(
    "hi",
    "Saluda a todos",
    "Interacción",
    false,
    "**$User** le dice hola a **$Target**",
    "**$User** saluda a todos"
);

inventory::submit! {
    CommandRegistration {
        command: &HI
    }
}

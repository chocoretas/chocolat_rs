use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SCARED: InteractionCommand = InteractionCommand::new(
    "scared",
    "Muestra tu miedo o asústate de alguien",
    "Interacción",
    false,
    "A **$User** le aterroriza **$Target** D:",
    "**$User** tiene miedo de algo D:"
);

inventory::submit! {
    CommandRegistration {
        command: &SCARED
    }
}

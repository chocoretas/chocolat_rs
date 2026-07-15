use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SCARED: InteractionCommand = InteractionCommand::new(
    "scared",
    "Muestra tu miedo o asústate de alguien",
    "Interacción",
    false,
    0xB22DFE,
    "**$User** Tiene miedo de algo. D:",
    "**$User** Tiene miedo de algo. D:"
);

inventory::submit! {
    CommandRegistration {
        command: &SCARED
    }
}

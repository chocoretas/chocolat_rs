use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static RUN: InteractionCommand = InteractionCommand::new(
    "run",
    "Corre por su vida",
    "Interacción",
    false,
    "**$User** corre lejos de **$Target**",
    "**$User** está corriendo por su vida"
);

inventory::submit! {
    CommandRegistration {
        command: &RUN
    }
}

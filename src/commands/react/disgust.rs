use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DISGUST: InteractionCommand = InteractionCommand::new(
    "disgust",
    "Muestra tu disgusto hacia algo o alguien",
    "Reacción",
    false,
    "A **$User** no le agrada **$Target** e.e",
    "**$User** esta disgustado."
);

inventory::submit! {
    CommandRegistration {
        command: &DISGUST
    }
}

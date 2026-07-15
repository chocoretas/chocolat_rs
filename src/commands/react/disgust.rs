use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DISGUST: InteractionCommand = InteractionCommand::new(
    "disgust",
    "Muestra tu disgusto hacia algo o alguien",
    "Reacción",
    false,
    "**$User** está disgustado.",
    "**$User** está disgustado."
);

inventory::submit! {
    CommandRegistration {
        command: &DISGUST
    }
}

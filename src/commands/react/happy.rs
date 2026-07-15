use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HAPPY: InteractionCommand = InteractionCommand::new(
    "happy",
    "Muestra tu felicidad",
    "Reacción",
    false,
    "**$User** esta feliz con **$Target** :D/",
    "**$User** esta feliz :D/"
);

inventory::submit! {
    CommandRegistration {
        command: &HAPPY
    }
}

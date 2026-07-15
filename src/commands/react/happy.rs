use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HAPPY: InteractionCommand = InteractionCommand::new(
    "happy",
    "Muestra tu felicidad",
    "Reacción",
    false,
    0x25B34C,
    "**$User** está feliz :D/",
    "**$User** está feliz :D/"
);

inventory::submit! {
    CommandRegistration {
        command: &HAPPY
    }
}

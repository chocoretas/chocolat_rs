use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SLEEP: InteractionCommand = InteractionCommand::new(
    "sleep",
    "Se va a dormir",
    "Interacción",
    false,
    "**$User** se va a dormir con **$Target**",
    "**$User** se va a dormir, buenas noches"
);

inventory::submit! {
    CommandRegistration {
        command: &SLEEP
    }
}

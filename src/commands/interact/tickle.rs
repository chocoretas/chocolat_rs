use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static TICKLE: InteractionCommand = InteractionCommand::new(
    "tickle",
    "Hazle cosquillas a alguien",
    "Interacción",
    true,
    "**$User** le hace cosquillas a **$Target**",
    "**$User** hace cosquillas"
);

inventory::submit! {
    CommandRegistration {
        command: &TICKLE
    }
}

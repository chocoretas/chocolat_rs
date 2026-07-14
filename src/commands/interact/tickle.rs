use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static TICKLE: InteractionCommand = InteractionCommand::new(
    "tickle",
    "Le hace cosquillas a alguien",
    "Interacción",
    true,
    "**$User** le hace cosquillas a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &TICKLE
    }
}

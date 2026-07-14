use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CHEEKS: InteractionCommand = InteractionCommand::new(
    "cheeks",
    "Le pellizca las mejillas a alguien",
    "Interacción",
    true,
    "**$User** le pellizca las mejillas a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &CHEEKS
    }
}

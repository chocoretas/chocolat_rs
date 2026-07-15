use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static COOKIE: InteractionCommand = InteractionCommand::new(
    "cookie",
    "Regala una galleta a alguien",
    "Interacción",
    true,
    "**$User** le regaló una galleta a **$Target** 🍪",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &COOKIE
    }
}

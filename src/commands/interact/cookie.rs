use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static COOKIE: InteractionCommand = InteractionCommand::new(
    "cookie",
    "Regala una galleta a alguien",
    "Interacción",
    true,
    0xFFDD2C,
    "**$Target,** has recibido una :cookie: de **$User**",
    "**$User** tiene una :cookie:"
);

inventory::submit! {
    CommandRegistration {
        command: &COOKIE
    }
}

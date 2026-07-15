use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HIGHFIVE: InteractionCommand = InteractionCommand::new(
    "highfive",
    "Choca los 5 con alguien",
    "Interacción",
    true,
    "¡**$User** choca esos 5 con **$Target**! 🙌",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &HIGHFIVE
    }
}

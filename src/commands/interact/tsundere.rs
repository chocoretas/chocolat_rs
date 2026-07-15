use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static TSUNDERE: InteractionCommand = InteractionCommand::new(
    "tsundere",
    "Sé tsundere con alguien",
    "Interacción",
    true,
    "¬¬" ¡Hmm! tonto, **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &TSUNDERE
    }
}

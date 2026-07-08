use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BAKA: InteractionCommand = InteractionCommand::new(
    "baka",
    "erm le dices baka a alguien creo",
    "Interacción",
    true,
    "¡¡B-BAAAKAAAA, **$Target**!!",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &BAKA
    }
}

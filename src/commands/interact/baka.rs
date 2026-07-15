use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BAKA: InteractionCommand = InteractionCommand::new(
    "baka",
    "Dile al usuario mencionado que es un completo Baka (idiota)",
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

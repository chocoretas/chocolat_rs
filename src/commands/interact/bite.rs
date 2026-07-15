use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BITE: InteractionCommand = InteractionCommand::new(
    "bite",
    "Muerde al usuario mencionado, o deja que Chocolat te muerda.",
    "Interacción",
    false,
    "**$User** ha mordido a **$Target** >n<",
    "ñam... *muerde a **$User***"
);

inventory::submit! {
    CommandRegistration {
        command: &BITE
    }
}

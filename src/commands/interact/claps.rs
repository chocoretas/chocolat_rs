use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CLAPS: InteractionCommand = InteractionCommand::new(
    "claps",
    "Aplaudir por algo o alguien",
    "Interacción",
    false,
    "**$User** le aplaude a **$Target** 👏",
    "**$User** comenzó a aplaudir 👏"
);

inventory::submit! {
    CommandRegistration {
        command: &CLAPS
    }
}

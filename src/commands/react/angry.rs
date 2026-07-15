use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static ANGRY: InteractionCommand = InteractionCommand::new(
    "angry",
    "Muestra tu enfado o enfádate con alguien",
    "Interacción",
    false,
    "**$User** se ha enfadado con **$Target** >:C",
    "**$User** se ha enfadado >:C"
);

inventory::submit! {
    CommandRegistration {
        command: &ANGRY
    }
}

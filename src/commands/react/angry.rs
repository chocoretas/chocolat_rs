use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static ANGRY: InteractionCommand = InteractionCommand::new(
    "angry",
    "Muestra tu enfado o enfádate con alguien",
    "Reacción",
    false,
    0xD40707,
    "**$User** está enfadado >:C",
    "**$User** está enfadado >:C"
);

inventory::submit! {
    CommandRegistration {
        command: &ANGRY
    }
}

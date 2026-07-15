use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static FACEPALM: InteractionCommand = InteractionCommand::new(
    "facepalm",
    "Haz un facepalm por algo absurdo",
    "Reacción",
    false,
    0x0F7AFC,
    "**$User** está decepcionado.",
    "**$User** está decepcionado."
);

inventory::submit! {
    CommandRegistration {
        command: &FACEPALM
    }
}

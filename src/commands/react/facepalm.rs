use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static FACEPALM: InteractionCommand = InteractionCommand::new(
    "facepalm",
    "Haz un facepalm por algo absurdo",
    "Reacción",
    false,
    "**$User** hace un facepalm por **$Target**",
    "**$User** se ha decepcionado..."
);

inventory::submit! {
    CommandRegistration {
        command: &FACEPALM
    }
}

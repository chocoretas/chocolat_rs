use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BLUSH: InteractionCommand = InteractionCommand::new(
    "blush",
    "Sonrojate o muestra tu pena",
    "Reacción",
    false,
    "**$User** se sonrojo por **$Target** o///O",
    "**$User** esta sonrojado o///O"
);

inventory::submit! {
    CommandRegistration {
        command: &BLUSH
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BLUSH: InteractionCommand = InteractionCommand::new(
    "blush",
    "Sonrojate o muestra tu pena",
    "Reacción",
    false,
    0x86C540,
    "**$User** está sonrojado o///O",
    "**$User** está sonrojado o///O"
);

inventory::submit! {
    CommandRegistration {
        command: &BLUSH
    }
}

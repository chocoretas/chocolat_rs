use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SLEEP: InteractionCommand = InteractionCommand::new(
    "sleep",
    "Vete a dormir o duerme junto a alguien",
    "Interacción",
    false,
    0x51F514,
    "**$User** tiene sueño o ya se durmió... zZz",
    "**$User** tiene sueño o ya se durmió... zZz"
);

inventory::submit! {
    CommandRegistration {
        command: &SLEEP
    }
}

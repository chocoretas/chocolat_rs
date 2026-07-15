use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static LICK: InteractionCommand = InteractionCommand::new(
    "lick",
    "Lame a un usuario",
    "Interacción",
    true,
    "**$User** lamió a **$Target** o///o",
    "**$User** lamió a alguien o///o"
);

inventory::submit! {
    CommandRegistration {
        command: &LICK
    }
}

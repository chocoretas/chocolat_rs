use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static LICK: InteractionCommand = InteractionCommand::new(
    "lick",
    "Lame a un usuario",
    "Interacción",
    true,
    "**$User** lamió a **$Target** :P",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &LICK
    }
}

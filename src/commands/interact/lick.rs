use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static LICK: InteractionCommand = InteractionCommand::new(
    "lick",
    "Lame a alguien",
    "Interacción",
    true,
    "**$User** lame a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &LICK
    }
}

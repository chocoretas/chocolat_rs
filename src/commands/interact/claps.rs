use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CLAPS: InteractionCommand = InteractionCommand::new(
    "claps",
    "Aplaude por algo",
    "Interacción",
    false,
    "**$User** le aplaude a **$Target**",
    "**$User** está aplaudiendo"
);

inventory::submit! {
    CommandRegistration {
        command: &CLAPS
    }
}

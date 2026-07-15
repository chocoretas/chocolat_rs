use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HUG: InteractionCommand = InteractionCommand::new(
    "hug",
    "abraza a otro usuario",
    "Interacción",
    true,
    0x79BD33,
    "**$User** le dio un abrazo a **$Target**",
    "**$User** da un abrazo"
);

inventory::submit! {
    CommandRegistration {
        command: &HUG
    }
}

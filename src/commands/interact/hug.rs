use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HUG: InteractionCommand = InteractionCommand::new(
    "hug",
    "abraza a otro usuario",
    "Interacción",
    true,
    "**$User** le dio un abrazo a **$Target** owo",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &HUG
    }
}

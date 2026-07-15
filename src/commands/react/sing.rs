use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SING: InteractionCommand = InteractionCommand::new(
    "sing",
    "Ponte a cantar una cancion",
    "Reacción",
    false,
    "**$User** le canta a **$Target**",
    "**$User** esta cantando."
);

inventory::submit! {
    CommandRegistration {
        command: &SING
    }
}

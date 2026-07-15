use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DISCOURAGED: InteractionCommand = InteractionCommand::new(
    "discouraged",
    "Siente decaimiento o desanimo",
    "Reacción",
    false,
    "**$User** se siente decaido por **$Target** :(",
    "**$User** se siente decaido. :("
);

inventory::submit! {
    CommandRegistration {
        command: &DISCOURAGED
    }
}

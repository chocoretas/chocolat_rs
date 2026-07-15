use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DISCOURAGED: InteractionCommand = InteractionCommand::new(
    "discouraged",
    "Siente decaimiento o desanimo",
    "Reacción",
    false,
    0xA4CBDC,
    "**$User** se siente decaído. :(",
    "**$User** se siente decaído. :("
);

inventory::submit! {
    CommandRegistration {
        command: &DISCOURAGED
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static ANGRY: InteractionCommand = InteractionCommand::new(
    "angry",
    "abraza a otro usuario",
    "Interacción",
    false,
    "**$User** se ha enfadado >:C",
    "**$User** se ha enfadado >:C" // Este code es basura pero soy muy flojo para decirle q no :wilted_rose:
);

inventory::submit! {
    CommandRegistration {
        command: &ANGRY
    }
}

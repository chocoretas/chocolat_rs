use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DANCE: InteractionCommand = InteractionCommand::new(
    "dance",
    "Baila alegremente",
    "Interacción",
    false,
    "**$User** baila con **$Target**",
    "**$User** se pone a bailar"
);

inventory::submit! {
    CommandRegistration {
        command: &DANCE
    }
}

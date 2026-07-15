use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SHRUG: InteractionCommand = InteractionCommand::new(
    "shrug",
    "Muestra que no lo sabes o no te importa",
    "Reacción",
    false,
    "A **$User** no le importa **$Target**",
    "**$User** no lo sabe..."
);

inventory::submit! {
    CommandRegistration {
        command: &SHRUG
    }
}

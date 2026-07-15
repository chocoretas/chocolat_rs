use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static NOPE: InteractionCommand = InteractionCommand::new(
    "nope",
    "Niega algo rotundamente",
    "Reacción",
    false,
    "**$User** le dice que NO a **$Target**",
    "**$User** dice que no rotundamente."
);

inventory::submit! {
    CommandRegistration {
        command: &NOPE
    }
}

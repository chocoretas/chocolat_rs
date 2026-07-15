use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static COOK: InteractionCommand = InteractionCommand::new(
    "cook",
    "Ponte a cocinar algo rico",
    "Interacción",
    false,
    0x97A4EF,
    "**$User** se ha puesto a cocinar.",
    "**$User** se ha puesto a cocinar."
);

inventory::submit! {
    CommandRegistration {
        command: &COOK
    }
}

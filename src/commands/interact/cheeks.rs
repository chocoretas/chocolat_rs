use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static CHEEKS: InteractionCommand = InteractionCommand::new(
    "cheeks",
    "Pellizca las mejillas de un usuario",
    "Interacción",
    false,
    "**$User** pellizca las mejillas de **$Target**",
    "**Chocolat** pellizca las mejillas de **$User**"
);

inventory::submit! {
    CommandRegistration {
        command: &CHEEKS
    }
}

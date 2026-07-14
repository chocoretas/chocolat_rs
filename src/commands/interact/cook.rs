use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static COOK: InteractionCommand = InteractionCommand::new(
    "cook",
    "Cocina algo rico",
    "Interacción",
    false,
    "**$User** está cocinando para **$Target**",
    "**$User** está cocinando algo delicioso"
);

inventory::submit! {
    CommandRegistration {
        command: &COOK
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static COOK: InteractionCommand = InteractionCommand::new(
    "cook",
    "Ponte a cocinar algo rico",
    "Interacción",
    false,
    "**$User** le preparó algo delicioso a **$Target** 🍳",
    "**$User** se ha puesto a cocinar 🍳"
);

inventory::submit! {
    CommandRegistration {
        command: &COOK
    }
}

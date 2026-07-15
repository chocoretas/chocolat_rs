use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BYE: InteractionCommand = InteractionCommand::new(
    "bye",
    "byebye",
    "Interacción",
    false,
    "**$User** acarició a **$Target** uwu",
    "**$User** se despide"
);

inventory::submit! {
    CommandRegistration {
        command: &BYE
    }
}

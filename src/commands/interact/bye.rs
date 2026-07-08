use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BYE: InteractionCommand = InteractionCommand::new(
    "bye",
    "byebye",
    "Interacción",
    false,
    "**$User** le dice adiós a **$Target**",
    "**$User** se retira del lugar, ¡adiós!"
);

inventory::submit! {
    CommandRegistration {
        command: &BYE
    }
}

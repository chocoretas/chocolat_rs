use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SPLASH: InteractionCommand = InteractionCommand::new(
    "splash",
    "Moja a alguien con agua",
    "Interacción",
    true,
    "**$User** moja a **$Target**",
    "**$User** salpica agua"
);

inventory::submit! {
    CommandRegistration {
        command: &SPLASH
    }
}

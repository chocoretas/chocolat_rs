use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SPLASH: InteractionCommand = InteractionCommand::new(
    "splash",
    "Salpica agua",
    "Interacción",
    true,
    "**$User** salpica agua a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &SPLASH
    }
}

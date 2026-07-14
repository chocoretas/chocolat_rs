use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SCARED: InteractionCommand = InteractionCommand::new(
    "scared",
    "Tiene miedo",
    "Interacción",
    false,
    "**$User** se asusta de **$Target**",
    "**$User** tiene mucho miedo"
);

inventory::submit! {
    CommandRegistration {
        command: &SCARED
    }
}

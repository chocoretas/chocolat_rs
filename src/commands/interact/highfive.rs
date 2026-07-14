use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static HIGHFIVE: InteractionCommand = InteractionCommand::new(
    "highfive",
    "Choca los cinco",
    "Interacción",
    true,
    "**$User** le choca los cinco a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &HIGHFIVE
    }
}

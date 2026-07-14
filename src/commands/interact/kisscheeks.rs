use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KISSCHEEKS: InteractionCommand = InteractionCommand::new(
    "kisscheeks",
    "Le da un beso en la mejilla a alguien",
    "Interacción",
    true,
    "**$User** le da un beso en la mejilla a **$Target**",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &KISSCHEEKS
    }
}

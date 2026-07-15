use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static RUN: InteractionCommand = InteractionCommand::new(
    "run",
    "Corre por tu vida o huye con alguien",
    "Interacción",
    false,
    "**$User** huye junto a **$Target** 🏃",
    "**$User** se echó a correr 🏃"
);

inventory::submit! {
    CommandRegistration {
        command: &RUN
    }
}

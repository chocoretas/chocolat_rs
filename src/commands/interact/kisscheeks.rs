use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KISSCHEEKS: InteractionCommand = InteractionCommand::new(
    "kisscheeks",
    "Besa las mejillas de alguien",
    "Interacción",
    true,
    "**$User** le dio un beso en la mejilla a **$Target** n.n",
    ""
);

inventory::submit! {
    CommandRegistration {
        command: &KISSCHEEKS
    }
}

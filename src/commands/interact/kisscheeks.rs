use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static KISSCHEEKS: InteractionCommand = InteractionCommand::new(
    "kisscheeks",
    "Besa las mejillas de alguien",
    "Interacción",
    true,
    0xCA3163,
    "**$User** le dio un beso en la mejilla a **$Target** n.n",
    "**$User** besa mejillas n.n"
);

inventory::submit! {
    CommandRegistration {
        command: &KISSCHEEKS
    }
}

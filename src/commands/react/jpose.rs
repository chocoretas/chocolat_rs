use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static JPOSE: InteractionCommand = InteractionCommand::new(
    "jpose",
    "Posa con estilo como en JoJos",
    "Reacción",
    false,
    "**$User** se puso a posar como los Jojo's.",
    "**$User** se puso a posar como los Jojo's."
);

inventory::submit! {
    CommandRegistration {
        command: &JPOSE
    }
}

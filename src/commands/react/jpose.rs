use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static JPOSE: InteractionCommand = InteractionCommand::new(
    "jpose",
    "Posa con estilo como en JoJos",
    "Reacción",
    false,
    "**$User** le hace una pose de JoJos a **$Target**",
    "**$User** se puso a posar como los JoJos."
);

inventory::submit! {
    CommandRegistration {
        command: &JPOSE
    }
}

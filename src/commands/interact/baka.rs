use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BAKA: InteractionCommand = InteractionCommand::new(
    "baka",
    "Dile al usuario mencionado que es un completo Baka (idiota)",
    "Interacción",
    true,
    0xD1FCF1,
    "**$Target**, BAKA!!",
    "**$User**, BAKA!!"
);

inventory::submit! {
    CommandRegistration {
        command: &BAKA
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static BOOM: InteractionCommand = InteractionCommand::new(
    "boom",
    "Explota de la emocion o enojo",
    "Reacción",
    false,
    0x46C78D,
    "**¡BOOM!**",
    "**¡BOOM!**"
);

inventory::submit! {
    CommandRegistration {
        command: &BOOM
    }
}

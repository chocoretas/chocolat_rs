use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SIP: InteractionCommand = InteractionCommand::new(
    "sip",
    "Bebe aca algo no se",
    "Interacción",
    false,
    0x6AC629,
    "**$User** Está bebiendo de forma atenta.",
    "**$User** Está bebiendo de forma atenta."
);

inventory::submit! {
    CommandRegistration {
        command: &SIP
    }
}

use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SIP: InteractionCommand = InteractionCommand::new(
    "sip",
    "Bebe aca algo no se",
    "Interacción",
    false,
    "**$User** está bebiendo de forma atenta.",
    "**$User** está bebiendo de forma atenta."
);

inventory::submit! {
    CommandRegistration {
        command: &SIP
    }
}

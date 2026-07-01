use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static SIP: InteractionCommand = InteractionCommand::new(
    "sip",
    "Alimenta a otro usuario",
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

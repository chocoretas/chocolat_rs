use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static FEED: InteractionCommand = InteractionCommand::new(
    "feed",
    "Dale de comer a un usuario, o deja que Chocolat te dé de comer.",
    "Interacción",
    false,
    "Parece que tienes hambre, **$Target**, ten algo de comida n.n",
    "**$User** está comiendo."
);

inventory::submit! {
    CommandRegistration {
        command: &FEED
    }
}

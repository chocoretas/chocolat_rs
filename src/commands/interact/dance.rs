use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DANCE: InteractionCommand = InteractionCommand::new(
    "dance",
    "Ponte a bailar o baila con alguien",
    "Interacción",
    false,
    "**$User** misteriosamente se puso a bailar o.o",
    "**$User** misteriosamente se puso a bailar o.o"
);

inventory::submit! {
    CommandRegistration {
        command: &DANCE
    }
}

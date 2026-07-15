use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DAB: InteractionCommand = InteractionCommand::new(
    "dab",
    "Haz un dab genial",
    "Reacción",
    false,
    0x86C540,
    "**$User** hizo un dab",
    "**$User** hizo un dab"
);

inventory::submit! {
    CommandRegistration {
        command: &DAB
    }
}

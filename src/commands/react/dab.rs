use crate::commands::interaction::InteractionCommand;
use crate::command::CommandRegistration;
use serenity::all::Colour;

pub static DAB: InteractionCommand = InteractionCommand::new(
    "dab",
    "Haz un dab genial",
    "Reacción",
    false,
    "**$User** le hizo un dab a **$Target**",
    "**$User** hizo un dab."
);

inventory::submit! {
    CommandRegistration {
        command: &DAB
    }
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component)]
pub struct Sprinting;

#[derive(Component)]
pub struct Exhausted {
    // track when exhaustion should end
    pub timer: Timer,
}

mod camera;
mod movement;
mod player;

use camera::CameraPlugin;
use player::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            PlayerPlugin,
        ))
        .run();
}

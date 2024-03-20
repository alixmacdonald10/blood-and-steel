use bevy::prelude::*;
use bevy::window::PrimaryWindow;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>) {
    
    let window = q_window.get_single().unwrap();

    commands.spawn(
         Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
         } 
    ); 
}

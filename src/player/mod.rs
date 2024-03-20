use std::time::Duration;

use crate::movement::{
    Sprinting,
    Exhausted
};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SPRITE_SIZE: f32 = 25.0; 
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SPRINT_SPEED: f32 = 300.0;
const PLAYER_STARTING_STAMINA: f32 = 2.0; // defined in seconds of sprinting
const PLAYER_STAMINA_REGEN: f32 = 0.5;  // define in seconds of sprinting per second
const PLAYER_EXHAUSTION: u64 = 1;  // defined in seconds of exhaustion

const CONTROLS_SPRINT_KEY: KeyCode = KeyCode::ShiftLeft;
const CONTROLS_MOVE_FORWARDS: KeyCode = KeyCode::KeyW;
const CONTROLS_MOVE_BACKWARDS: KeyCode = KeyCode::KeyS; 
const CONTROLS_MOVE_LEFT: KeyCode = KeyCode::KeyA;
const CONTROLS_MOVE_RIGHT: KeyCode = KeyCode::KeyD;


#[derive(Component)]
struct Player {
    movement_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            movement_speed: PLAYER_SPEED
        }
    }
}

#[derive(Component)]
struct Stamina {
    max: f32,
    current: f32,
    
}

impl Default for Stamina {
    fn default() -> Self {
        Stamina {
            max: PLAYER_STARTING_STAMINA,
            current: PLAYER_STARTING_STAMINA,
        }
    }
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (handle_exhaustion, handle_player_sprinting, player_movement));
    }
}

fn spawn_player(mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>) {
    
    let window = q_window.get_single().unwrap();

    commands.spawn((
        SpriteBundle { 
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE)),
                ..Default::default()
            },
            ..default()
        },
        Player::default(), 
        Stamina::default(),
    ));
}

fn handle_player_sprinting(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(Entity, &mut Player, &mut Stamina, Option<&Sprinting>, Option<&Exhausted>)>, 
) {
    if let Ok((entity, mut player, stamina, sprinting, exhausted)) = q_player.get_single_mut() {
        match (sprinting, exhausted) {
            (Some(_), None) => {
                // sprinting
                if stamina.current <= 0.0 {
                    remove_sprinting(&mut commands, entity, &mut player);
                    add_exhaustion(&mut commands, entity);
                } else if keyboard_input.just_released(CONTROLS_SPRINT_KEY) {
                    remove_sprinting(&mut commands, entity, &mut player);
                }
            },
            (None, None) => {
                // not sprinting and not exhausted
                if keyboard_input.pressed(CONTROLS_SPRINT_KEY) {
                    add_sprinting(&mut commands, entity, &mut player);
               }
            },
            (_, _) => {},
        }
    }
}

fn remove_sprinting(commands: &mut Commands, entity: Entity, player: &mut Player) {
    player.movement_speed = PLAYER_SPEED; 
    commands.entity(entity).remove::<Sprinting>();
     info!("removed SPRINTING: Speed {}", player.movement_speed);
}

fn add_sprinting(commands: &mut Commands, entity: Entity, player: &mut Player) {
    player.movement_speed = PLAYER_SPRINT_SPEED; 
    commands.entity(entity).insert(Sprinting);
    info!("added SPRINTING: Speed {}", player.movement_speed); 
}

fn add_exhaustion(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(Exhausted {
        timer: Timer::new(Duration::from_secs(PLAYER_EXHAUSTION), TimerMode::Once)
    });
    info!("EXHAUSTED!"); 
}

fn handle_exhaustion(
    mut commands: Commands,
    mut q_exhausted_player: Query<(Entity, &mut Exhausted, &Player)>,
    time: Res<Time>,
) {
    for (entity, mut exhaustion_timer, player) in q_exhausted_player.iter_mut() {
        // timers gotta be ticked, to work
        exhaustion_timer.timer.tick(time.delta());

        // if it finished, remove the exhaustion
        if exhaustion_timer.timer.finished() {
            info!("RECOVERED!");
            commands.entity(entity).remove::<Exhausted>();
        }
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&Player, &mut Transform, &mut Stamina, Option<&Sprinting>, Option<&Exhausted>)>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform, mut stamina, sprinting, exhausted)) = q_player.get_single_mut() {
        let mut direction: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(CONTROLS_MOVE_LEFT) {
            // left movement
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(CONTROLS_MOVE_RIGHT) { 
            // right movement
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(CONTROLS_MOVE_FORWARDS) {
            // forwards movement
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(CONTROLS_MOVE_BACKWARDS) {
            // backwards movement
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        
        if direction.length() > 0.0 {
            // movement has happened            
            direction = direction.normalize();
            if let Some(_) = sprinting { 
                stamina_reduction(&mut stamina, &time);
            }
        }

        transform.translation += direction * player.movement_speed * time.delta_seconds();
        if let (None, None) = (sprinting, exhausted) {
            if stamina.current < stamina.max {
                stamina_regen(&mut stamina, &time);
            }
        }
        
    }
}

fn stamina_reduction(stamina: &mut Stamina, time: &Res<Time>) {

    let stamina_update = stamina.current - time.delta_seconds();
    if stamina_update < 0.0 {
        stamina.current = 0.0;
    } else {
        stamina.current = stamina_update;
    }
    info!("Stamina DRAIN: {}", stamina.current)
}


fn stamina_regen(stamina: &mut Stamina, time: &Res<Time>) {
    let stamina_update = stamina.current + PLAYER_STAMINA_REGEN * time.delta_seconds();
    if stamina_update >= stamina.max {
        stamina.current = stamina.max;
    } else {
       stamina.current = stamina_update; 
       info!("Stamina regen: {}", stamina.current)
    }
}


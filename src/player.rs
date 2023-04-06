use bevy::{prelude::*, window::PrimaryWindow};

use crate::GameState;

const PLAYER_SPEED: f32 = 100.0;
const BOTTOM_OFFSET: f32 = 30.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Spawning)))
            .add_systems((move_player,).in_set(OnUpdate(GameState::Next)));
    }
}

fn spawn_player(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ALICE_BLUE,
                custom_size: Some(Vec2::new(20.0, 30.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -window.height() / 2.0 + BOTTOM_OFFSET, 0.0),
            ..Default::default()
        },
        Player,
        Name::from("Player"),
    ));
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut transform = player.single_mut();
    let window = window.single();

    if keyboard.pressed(KeyCode::A) && transform.translation.x >= -window.width() / 2.0 + 10.0 {
        transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::D) && transform.translation.x <= window.width() / 2.0 - 10.0 {
        transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
}

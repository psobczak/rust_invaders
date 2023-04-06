use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};

use crate::{AnimationIndices, AnimationTimer, GameState, Grid, GridPosition, MyAssets};

pub struct InvaderPlugin;

#[derive(Component)]
pub struct Invader;

#[derive(Component, Default)]
enum InvaderState {
    #[default]
    Moving,
    Dying,
}

impl InvaderState {
    fn get_animation_indices(&self) -> AnimationIndices {
        match self {
            InvaderState::Moving => AnimationIndices { start: 0, end: 1 },
            InvaderState::Dying => todo!(),
        }
    }
}

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridPosition>()
            .add_system((spawn_invaders).in_schedule(OnEnter(GameState::Spawning)))
            .add_systems(
                (
                    position_invaders_on_grid,
                    animate_invaders,
                    change_grid_position,
                )
                    .in_set(OnUpdate(GameState::Next)),
            );
    }
}

fn spawn_invaders(mut commands: Commands, grid: Res<Grid>, assets: Res<MyAssets>) {
    commands
        .spawn((Name::from("Invaders"), SpatialBundle::default()))
        .with_children(|children| {
            for column in 2..grid.columns - 2 {
                for row in 1..8 {
                    children.spawn((
                        GridPosition { x: column, y: row },
                        SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: 0,
                                anchor: Anchor::TopLeft,
                                ..Default::default()
                            },
                            texture_atlas: assets.invaders.clone(),
                            ..Default::default()
                        },
                        AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                        Invader,
                        InvaderState::default(),
                    ));
                }
            }
        });
}

fn position_invaders_on_grid(
    mut invaders: Query<(
        &mut Transform,
        &GridPosition,
        Changed<GridPosition>,
        With<Invader>,
    )>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    let start_x = -window.width() / 2.0;
    let start_y = window.height() / 2.0;
    for (mut transform, grid_position, _, _) in &mut invaders {
        *transform = grid_position.get_transform(start_x, start_y);
    }
}

fn animate_invaders(
    mut invaders: Query<
        (&mut AnimationTimer, &mut TextureAtlasSprite, &InvaderState),
        With<Invader>,
    >,
    time: Res<Time>,
) {
    for (mut timer, mut sprite, state) in &mut invaders {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            let animation_indices = state.get_animation_indices();
            sprite.index = if sprite.index == animation_indices.end {
                animation_indices.start
            } else {
                sprite.index + 1
            };
        }
    }
}

fn change_grid_position(
    keyboard: Res<Input<KeyCode>>,
    mut grid_position: Query<&mut GridPosition>,
) {
    for mut position in &mut grid_position {
        if keyboard.just_pressed(KeyCode::Left) {
            position.x -= 1;
        }

        if keyboard.just_pressed(KeyCode::Right) {
            position.x += 1;
        }

        if keyboard.just_pressed(KeyCode::Down) {
            position.y += 1;
        }

        if keyboard.just_pressed(KeyCode::Up) {
            position.y -= 1;
        }
    }
}

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AnimationIndices, AnimationTimer, GameState, Grid, GridPosition, Invader, MyAssets, Unit};

use super::{InvaderCount, MoveTimer, UnitBundle};

use super::Direction;

#[derive(Resource)]
struct TimerConfig {
    seconds: f32,
    min_seconds: f32,
    decrement: f32,
}

pub struct InvaderPlugin;

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

enum EdgeReached {
    Left,
    Right,
}

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<EdgeReached>()
            .insert_resource(TimerConfig {
                seconds: 1.0,
                min_seconds: 0.5,
                decrement: 0.1,
            })
            .add_system((spawn_invaders).in_schedule(OnEnter(GameState::Spawning)))
            .add_systems(
                (
                    position_invaders_on_grid,
                    animate_invaders,
                    move_invaders,
                    detect_edge,
                    change_moving_direction,
                )
                    .in_set(OnUpdate(GameState::Next)),
            );
    }
}

fn spawn_invaders(
    mut commands: Commands,
    grid: Res<Grid>,
    assets: Res<MyAssets>,
    timer_config: Res<TimerConfig>,
) {
    let mut invader_count = InvaderCount(0);
    commands
        .spawn((Name::from("Invaders"), SpatialBundle::default()))
        .with_children(|children| {
            for column in 2..grid.columns - 2 {
                for row in 2..8 {
                    children.spawn((
                        UnitBundle::new(
                            column,
                            row,
                            assets.invaders.clone(),
                            100,
                            timer_config.seconds,
                        ),
                        Invader,
                        InvaderState::default(),
                    ));
                    invader_count.0 += 1;
                }
            }
        });

    commands.insert_resource(invader_count);
}

fn position_invaders_on_grid(
    mut invaders: Query<(
        &mut Transform,
        &GridPosition,
        Changed<GridPosition>,
        With<Unit>,
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
        With<Unit>,
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

fn detect_edge(
    invaders: Query<(&GridPosition, Changed<GridPosition>, With<Invader>)>,
    grid: Res<Grid>,
    mut writer: EventWriter<EdgeReached>,
) {
    for (grid_position, _, _) in &invaders {
        if grid_position.x == 0 {
            writer.send(EdgeReached::Left)
        }

        if grid_position.x == grid.columns - 1 {
            writer.send(EdgeReached::Right)
        }
    }
}

fn change_moving_direction(
    mut direction: Query<&mut Direction, With<Invader>>,
    mut reader: EventReader<EdgeReached>,
) {
    for event in reader.iter() {
        for mut direction in &mut direction {
            match event {
                EdgeReached::Left => *direction = Direction::Right,
                EdgeReached::Right => *direction = Direction::Left,
            }
        }
    }
}

fn move_invaders(
    mut invaders: Query<(&mut GridPosition, &mut MoveTimer, &Direction)>,
    time: Res<Time>,
) {
    for (mut grid_position, mut move_timer, direction) in &mut invaders {
        move_timer.0.tick(time.delta());

        if move_timer.0.just_finished() {
            match direction {
                Direction::Left => grid_position.x -= 1,
                Direction::Right => grid_position.x += 1,
            }
        }
    }
}

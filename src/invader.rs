use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AnimationIndices, AnimationTimer, GameState, Grid, GridPosition, MyAssets, Worth};

pub struct InvaderPlugin;

#[derive(Resource)]
pub struct InvaderCount(pub usize);

#[derive(Component)]
pub struct Invader;

#[derive(Resource)]
struct TimerConfig {
    seconds: f32,
    min_seconds: f32,
    decrement: f32,
}

#[derive(Component, Default)]
enum InvaderState {
    #[default]
    Moving,
    Dying,
}

#[derive(Component, Default)]
enum Direction {
    #[default]
    Left,
    Right,
}

enum EdgeReached {
    Left,
    Right,
}

#[derive(Component)]
struct MoveTimer(Timer);

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridPosition>()
            .insert_resource(TimerConfig {
                seconds: 1.0,
                min_seconds: 0.5,
                decrement: 0.1,
            })
            .add_event::<EdgeReached>()
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

#[derive(Bundle)]
struct InvaderBundle {
    grid_position: GridPosition,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    invader: Invader,
    state: InvaderState,
    direction: Direction,
    move_timer: MoveTimer,
    worth: Worth,
}

impl InvaderBundle {
    fn new(
        x: usize,
        y: usize,
        texture_atlas: Handle<TextureAtlas>,
        worth: usize,
        starting_seconds: f32,
    ) -> Self {
        Self {
            grid_position: GridPosition { x, y },
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas,
                ..Default::default()
            },
            animation_timer: AnimationTimer(Timer::from_seconds(
                starting_seconds,
                TimerMode::Repeating,
            )),
            invader: Invader,
            state: InvaderState::default(),
            direction: Direction::default(),
            move_timer: MoveTimer(Timer::from_seconds(starting_seconds, TimerMode::Repeating)),
            worth: Worth(worth),
        }
    }
}

impl InvaderState {
    fn get_animation_indices(&self) -> AnimationIndices {
        match self {
            InvaderState::Moving => AnimationIndices { start: 0, end: 1 },
            InvaderState::Dying => todo!(),
        }
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
                    children.spawn(InvaderBundle::new(
                        column,
                        row,
                        assets.invaders.clone(),
                        100,
                        timer_config.seconds,
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

fn detect_edge(
    invaders: Query<&GridPosition, Changed<GridPosition>>,
    grid: Res<Grid>,
    mut writer: EventWriter<EdgeReached>,
) {
    for grid_position in &invaders {
        if grid_position.x == 0 {
            writer.send(EdgeReached::Left)
        }

        if grid_position.x == grid.columns - 1 {
            writer.send(EdgeReached::Right)
        }
    }
}

fn change_moving_direction(
    mut direction: Query<&mut Direction>,
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

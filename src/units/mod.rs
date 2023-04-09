use bevy::prelude::*;

mod invader;
mod spaceship;

use crate::{AnimationTimer, GridPosition, Worth};

use self::{invader::InvaderPlugin, spaceship::SpaceshipPlugin};

#[derive(Resource)]
pub struct InvaderCount(pub usize);

#[derive(Component)]
struct MoveTimer(Timer);

#[derive(Component, Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Invader;

#[derive(Component)]
pub struct Starship;

#[derive(Bundle)]
pub struct UnitBundle {
    grid_position: GridPosition,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    direction: Direction,
    move_timer: MoveTimer,
    worth: Worth,
    unit: Unit,
}

impl UnitBundle {
    pub fn new(
        x: isize,
        y: isize,
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
            direction: Direction::default(),
            move_timer: MoveTimer(Timer::from_seconds(starting_seconds, TimerMode::Repeating)),
            worth: Worth(worth),
            unit: Unit,
        }
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InvaderPlugin).add_plugin(SpaceshipPlugin);
    }
}

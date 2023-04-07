mod invader;
mod player;
mod projectile;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use invader::*;
pub use player::*;
pub use projectile::*;

pub const CELL_SIZE: f32 = 32.0;

#[derive(Resource, Debug)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Reflect, Resource, Default, Component, Debug)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Spawning,
    Next,
}

#[derive(Component)]
pub struct AnimationTimer(Timer);

pub struct AnimationIndices {
    start: usize,
    end: usize,
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 1, rows = 2,))]
    #[asset(path = "invader.png")]
    pub invaders: Handle<TextureAtlas>,
}

impl GridPosition {
    fn get_transform(&self, start_x: f32, start_y: f32) -> Transform {
        let start_x = start_x;
        let x = start_x + (self.x as f32 * CELL_SIZE);
        let y = start_y - (self.y as f32 * CELL_SIZE);
        Transform::from_xyz(x, y, 0.0)
    }
}

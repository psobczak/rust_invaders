mod invader;
mod player;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use invader::*;
pub use player::*;

#[derive(Resource, Debug)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Reflect, Resource, Default, Component)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 1, rows = 2,))]
    #[asset(path = "invader.png")]
    pub invaders: Handle<TextureAtlas>,
}

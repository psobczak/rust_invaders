mod grid;
mod invader;
mod player;
mod projectile;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use grid::*;
pub use invader::*;
pub use player::*;
pub use projectile::*;

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

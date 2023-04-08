mod grid;
mod invader;
mod player;
mod projectile;
mod score;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use grid::*;
use score::*;

pub mod plugins {
    pub use super::invader::InvaderPlugin;
    pub use super::player::PlayerPlugin;
    pub use super::projectile::ProjectilePlugin;
    pub use super::score::ScorePlugin;
}

pub mod components {
    pub use super::grid::GridPosition;
    pub use super::invader::Invader;
    pub use super::player::Player;
    pub use super::score::Worth;
}

pub mod resources {
    pub use super::grid::Grid;
    pub use super::score::Score;
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Spawning,
    Next,
}

use bevy::prelude::*;

pub struct Cell;

impl Cell {
    pub const SIZE: f32 = 32.0;

    pub fn half_size() -> f32 {
        Self::SIZE / 2.0
    }
}

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

impl GridPosition {
    pub fn get_transform(&self, start_x: f32, start_y: f32) -> Transform {
        let start_x = start_x + Cell::half_size();
        let start_y = start_y - Cell::half_size();
        let x = start_x + (self.x as f32 * Cell::SIZE);
        let y = start_y - (self.y as f32 * Cell::SIZE);
        Transform::from_xyz(x, y, 0.0)
    }
}

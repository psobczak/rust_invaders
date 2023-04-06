use bevy::prelude::*;

use crate::{GameState, Grid, GridPosition, MyAssets};

pub struct InvaderPlugin;

#[derive(Component)]
struct AnimationTimer(Timer);

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridPosition>().add_system(
            spawn_invaders
                .run_if(resource_exists::<Grid>())
                .in_schedule(OnEnter(GameState::Next)),
        );
    }
}

fn spawn_invaders(mut commands: Commands, grid: Res<Grid>, assets: Res<MyAssets>) {
    commands
        .spawn((Name::from("Invaders"), SpatialBundle::default()))
        .with_children(|children| {
            for row in 0..grid.rows {
                for column in 0..grid.columns {
                    children.spawn((
                        GridPosition { x: column, y: row },
                        SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(0),
                            texture_atlas: assets.invaders.clone(),
                            ..Default::default()
                        },
                    ));
                }
            }
        });
}

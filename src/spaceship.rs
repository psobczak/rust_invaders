use bevy::{prelude::*, window::PrimaryWindow};

use crate::{despawn_entities, Cell, GameState, Grid, GridPosition, MyAssets, Worth};

const SPACESHIP_SPEED: f32 = 200.0;

pub struct SpaceshipPlugin;

#[derive(Resource)]
struct SpaceshipSpawnTimer(Timer);

#[derive(Component)]
pub struct Starship;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpaceshipSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            (
                spawn_spaceship,
                despawn_entities::<Starship>
                    .run_if(starship_outside.and_then(any_with_component::<Starship>())),
                move_spaceship.run_if(any_with_component::<Starship>()),
            )
                .in_set(OnUpdate(GameState::Next)),
        );
    }
}

fn spawn_spaceship(
    mut commands: Commands,
    grid: Res<Grid>,
    assets: Res<MyAssets>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut spawn_timer: ResMut<SpaceshipSpawnTimer>,
) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.just_finished() {
        let window = window.single();
        let grid_position = GridPosition {
            x: grid.columns + 1,
            y: 0,
        };
        let transform = grid_position.get_transform(-window.width() / 2.0, window.height() / 2.0);
        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: assets.invaders.clone(),
                transform,
                ..Default::default()
            },
            Worth(1000),
            Starship,
            Name::from("Starship"),
        ));
    }
}

fn starship_outside(
    starship: Query<&GlobalTransform, With<Starship>>,
    window: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    let window = window.single();
    for transform in &starship {
        if transform.translation().x < -window.width() / 2.0 - Cell::half_size() {
            return true;
        }
    }

    false
}

fn move_spaceship(mut ship: Query<&mut Transform, With<Starship>>, time: Res<Time>) {
    for mut transform in &mut ship {
        transform.translation.x -= time.delta_seconds() * SPACESHIP_SPEED
    }
}

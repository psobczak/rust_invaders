use bevy::{prelude::*, window::PrimaryWindow};

use crate::{GameState, Grid, GridPosition, MyAssets, Starship, UnitBundle};

pub struct SpaceshipPlugin;

#[derive(Resource)]
struct SpaceshipSpawnTimer(Timer);

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpaceshipSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            (
                spawn_spaceship,
                despawn_entities::<Starship>.run_if(starship_outside),
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
    mut spawn_timer: ResMut<SpaceshipSpawnTimer>,
) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.just_finished() {
        commands.spawn((
            UnitBundle::new(grid.columns + 1, 0, assets.invaders.clone(), 1000, 0.1),
            Starship,
            Name::from("Starship"),
        ));
    }
}

fn despawn_entities<C: Component>(mut commands: Commands, entities: Query<Entity, With<C>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn starship_outside(starship: Query<&GridPosition, With<Starship>>) -> bool {
    if let Ok(grid_position) = starship.get_single() {
        return grid_position.x < 0;
    }

    false
}

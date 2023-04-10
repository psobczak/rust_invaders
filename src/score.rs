use bevy::prelude::*;

use crate::{GameState, InvaderCount};

pub struct ScorePlugin;

#[derive(Component)]
pub struct Worth(pub usize);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Score(usize);

#[derive(Deref)]
pub struct ScoreIncreased(pub usize);

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreIncreased>()
            .insert_resource(Score::default())
            .add_systems(
                (
                    increase_sscore.run_if(on_event::<ScoreIncreased>()),
                    player_win.run_if(resource_exists_and_equals(InvaderCount(0))),
                )
                    .in_set(OnUpdate(GameState::Next)),
            );
    }
}

fn increase_sscore(mut score: ResMut<Score>, mut reader: EventReader<ScoreIncreased>) {
    for event in reader.iter() {
        **score += event.0;
    }
}

fn player_win(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::PlayerWon);
    println!("YOU WON!");
}

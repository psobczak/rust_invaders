use bevy::prelude::*;

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
            .add_system(increase_sscore.run_if(on_event::<ScoreIncreased>()));
    }
}

fn increase_sscore(mut score: ResMut<Score>, mut reader: EventReader<ScoreIncreased>) {
    for event in reader.iter() {
        **score += event.0;
    }
}

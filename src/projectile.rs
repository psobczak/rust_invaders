use bevy::{prelude::*, sprite::collide_aabb, window::PrimaryWindow};

use crate::{GameState, Invader, Player};

const PROJECTILE_SPPED: f32 = 200.0;

pub struct ProjectilePlugin;

#[derive(Component)]
struct Projectile;

struct HitDetected {
    invader: Entity,
    projectile: Entity,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitDetected>().add_systems(
            (
                spawn_projectile,
                move_projectile,
                despawn_projectiles,
                hit_invader,
                handle_hit,
            )
                .in_set(OnUpdate(GameState::Next)),
        );
    }
}

fn spawn_projectile(
    player: Query<(&GlobalTransform, &Player)>,
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
) {
    let (player_transform, player) = player.single();

    if keyboard.just_pressed(KeyCode::Space) && player.can_shoot {
        let player_translation = player_transform.translation();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(2.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    player_translation.x,
                    player_translation.y + 30.0,
                    0.0,
                ),
                ..Default::default()
            },
            Projectile,
        ));
    }
}

fn move_projectile(mut projectiles: Query<&mut Transform, With<Projectile>>, time: Res<Time>) {
    for mut transform in &mut projectiles {
        transform.translation.y += time.delta_seconds() * PROJECTILE_SPPED;
    }
}

fn despawn_projectiles(
    projectiles: Query<(&GlobalTransform, Entity), With<Projectile>>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    for (transform, projectile) in projectiles.iter() {
        if transform.translation().y > window.height() / 2.0 {
            commands.entity(projectile).despawn_recursive();
        }
    }
}

fn hit_invader(
    projectiles: Query<(&GlobalTransform, Entity), With<Projectile>>,
    invaders: Query<(&GlobalTransform, Entity), With<Invader>>,
    mut writer: EventWriter<HitDetected>,
) {
    for (projectile_transform, projectile) in &projectiles {
        for (invader_transform, invader) in &invaders {
            if collide_aabb::collide(
                projectile_transform.translation(),
                Vec2::new(2.0, 10.0),
                invader_transform.translation(),
                Vec2::new(32.0, 32.0),
            )
            .is_some()
            {
                writer.send(HitDetected {
                    invader,
                    projectile,
                })
            }
        }
    }
}

fn handle_hit(mut commands: Commands, mut reader: EventReader<HitDetected>) {
    for event in reader.iter() {
        commands.entity(event.projectile).despawn_recursive();
        commands.entity(event.invader).despawn_recursive();
    }
}

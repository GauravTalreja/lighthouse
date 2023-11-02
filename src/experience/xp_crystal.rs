use bevy::prelude::*;

use rand::Rng;

use crate::{
    constants::SCALING_VEC3,
    enemies::enemy::EnemyDeathEvent,
    loading::TextureAssets,
    movement::{
        edge_teleport::EdgeTeleports, fake_magnetic::FakeMagnetic, friction::Friction, velocity::Velocity,
    },
    player::Player,
    util::{
        radians::Radian,
        rng::{GlobalSeed, RNG},
    },
};

use super::experience::Experience;

#[derive(Resource)]
pub struct CrystalRNG(pub RNG);

#[derive(Component)]
pub struct XPCrystal;

pub fn create_xp_crystal_rng(seed: Res<GlobalSeed>, mut commands: Commands) {
    commands.insert_resource(CrystalRNG(RNG::new(seed.0.as_str(), "crystal_rng")))
}

#[derive(Bundle)]
pub struct XPCrystalBundle {
    pub sprite_bundle: SpriteBundle,
    pub crystal: XPCrystal,
    pub velocity: Velocity,
    pub friction: Friction,
    pub magnetic: FakeMagnetic,
}

pub fn drop_crystals(
    mut enemy_death_event: EventReader<EnemyDeathEvent>,
    mut crystal_rng: ResMut<CrystalRNG>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for death_ev in enemy_death_event.iter() {
        for _i in 0..death_ev.enemy.xp {
            let _rng = &mut crystal_rng.0 .0;
            let rng = &mut crystal_rng.0 .0;

            let _velocity: f32 = rng.gen_range(20.0..50.0);
            let _direction: Radian = Radian {
                angle: rng.gen_range(Radian::ZERO.angle..Radian::FULL.angle),
            };
            let velocity: f32 = rng.gen_range(20.0..50.0);
            let direction: Radian = Radian {
                angle: rng.gen_range(Radian::ZERO.angle..Radian::FULL.angle),
            };

            commands
                .spawn(XPCrystalBundle {
                    sprite_bundle: SpriteBundle {
                        transform: Transform {
                            translation: death_ev.location,
                            rotation: default(),
                            scale: SCALING_VEC3,
                        },
                        texture: textures.crystal.clone(),
                        ..Default::default()
                    },
                    crystal: XPCrystal,
                    velocity: (direction.unit_vector() * velocity).into(),
                    friction: Friction { force: 50.0 },
                    magnetic: FakeMagnetic { force: 1_000_000.0 },
                })
                .insert(EdgeTeleports);
        }
    }
}

pub fn xp_crystal_update(
    q_crystals: Query<(Entity, &Transform), (With<XPCrystal>, Without<Player>)>,
    mut q_player: Query<(&Transform, &mut Experience), (With<Player>, Without<XPCrystal>)>,
    mut commands: Commands,
) {
    let (_player_transform, _experience) = q_player.single_mut();
    let (player_transform, mut experience) = q_player.single_mut();

    for (entity, crystal_transform) in q_crystals.iter() {
        let _distance = crystal_transform
            .translation
            .distance(player_transform.translation);
        let distance = crystal_transform
            .translation
            .distance(player_transform.translation);
        if distance < experience.pick_distance {
            commands.entity(entity).despawn();
            experience.curr_experience += 1;
        }
    }
}

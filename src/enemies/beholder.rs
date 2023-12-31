use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_kira_audio::AudioControl;

use crate::{
    animation::{
        controller::AnimationController,
        info::{AnimationInfoBuilder, AnimationStateInfo},
        make_animation_bundle, Animation, AnimationStateChangeEvent, AnimationStateStorage,
    },
    audio::FXChannel,
    collision::collider::Collider,
    combat::{
        health::Health,
        healthbar::NeedsHealthBar,
        projectile::{DamageTarget, PiercingMode, Projectile},
        teams::{Team, TeamMember},
    },
    loading::{AudioAssets, TextureAssets},
    movement::velocity::Velocity,
    player::Player,
    util::radians::Radian,
};

use super::{
    ai::{ChargeShootEvent, MoveAndShootAI, ShootEvent},
    enemy::{Enemy, EnemyType},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BeholderAnimation {
    Flying,
    Shoot,
}

impl Animation<BeholderAnimation> for BeholderAnimation {
    fn get_states() -> Vec<AnimationStateInfo<BeholderAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(
                BeholderAnimation::Flying,
                16,
                Duration::from_secs_f32(1. / 8.),
            )
            .add_frames(
                BeholderAnimation::Shoot,
                6,
                Duration::from_secs_f32(1. / 8.),
            )
            .build()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BeholderProjectileAnimation {
    Flying,
}

impl Animation<BeholderProjectileAnimation> for BeholderProjectileAnimation {
    fn get_states() -> Vec<AnimationStateInfo<BeholderProjectileAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(
                BeholderProjectileAnimation::Flying,
                4,
                Duration::from_secs_f32(1. / 4.),
            )
            .build()
    }
}

#[derive(Component)]
pub struct BeholderPrince;

pub fn beholder_update(
    q_beholders: Query<
        (Entity, &Transform, &AnimationController<BeholderAnimation>),
        Without<Player>,
    >,
    q_beholder_prince: Query<&BeholderPrince>,
    q_player: Query<(Entity, &Transform), With<Player>>,
    mut shoot_ev: EventReader<ShootEvent>,
    mut charge_ev: EventReader<ChargeShootEvent>,
    mut animate: EventWriter<AnimationStateChangeEvent<BeholderAnimation>>,
    beholder_projetile_animations: Res<AnimationStateStorage<BeholderProjectileAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    fx: Res<FXChannel>,
    audio: Res<AudioAssets>,
    mut commands: Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.beholder_projectile.clone(),
        Vec2 { x: 32., y: 32. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for charge in charge_ev.iter() {
        if let Ok((entity, _, _)) = q_beholders.get(charge.entity) {
            animate.send(AnimationStateChangeEvent {
                id: entity,
                state_id: BeholderAnimation::Shoot,
            })
        }
    }

    for shoot in shoot_ev.iter() {
        if let Ok((entity, transform, _)) = q_beholders.get(shoot.entity) {
            animate.send(AnimationStateChangeEvent {
                id: entity,
                state_id: BeholderAnimation::Flying,
            });
            let (_player_entity, player_transform) = q_player.single();

            let direction =
                player_transform.translation.truncate() - transform.translation.truncate();
            // obtain angle to target with respect to x-axis.
            let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
            let direction_vec = angle_to_target.unit_vector();

            fx.play(audio.fireball.clone());

            commands
                .spawn(make_animation_bundle(
                    BeholderProjectileAnimation::Flying,
                    &beholder_projetile_animations,
                    texture_atlas_handle.clone(),
                    transform.translation,
                    1.,
                ))
                .insert(Projectile {
                    dmg: 1,
                    damage_target: DamageTarget::Team(Team::Player),
                    piercing_mode: PiercingMode::None,
                    entities_hit: vec![],
                    is_alive: true,
                })
                .insert(Velocity {
                    vec: direction_vec * 40.,
                })
                .insert(Collider::new_circle(15., transform.translation.truncate()));

            if let Ok(_) = q_beholder_prince.get(entity) {
                commands
                    .spawn(make_animation_bundle(
                        BeholderProjectileAnimation::Flying,
                        &beholder_projetile_animations,
                        texture_atlas_handle.clone(),
                        transform.translation,
                        1.,
                    ))
                    .insert(Projectile {
                        dmg: 1,
                        damage_target: DamageTarget::Team(Team::Player),
                        piercing_mode: PiercingMode::None,
                        entities_hit: vec![],
                        is_alive: true,
                    })
                    .insert(Velocity {
                        vec: (angle_to_target + Radian::from_degrees(10.)).unit_vector() * 40.,
                    })
                    .insert(Collider::new_circle(15., transform.translation.truncate()));

                commands
                    .spawn(make_animation_bundle(
                        BeholderProjectileAnimation::Flying,
                        &beholder_projetile_animations,
                        texture_atlas_handle.clone(),
                        transform.translation,
                        1.,
                    ))
                    .insert(Projectile {
                        dmg: 1,
                        damage_target: DamageTarget::Team(Team::Player),
                        piercing_mode: PiercingMode::None,
                        entities_hit: vec![],
                        is_alive: true,
                    })
                    .insert(Velocity {
                        vec: (angle_to_target - Radian::from_degrees(10.)).unit_vector() * 40.,
                    })
                    .insert(Collider::new_circle(15., transform.translation.truncate()));
            }
        }
    }
}

pub fn spawn_beholder(
    position: Vec3,
    animations: &Res<AnimationStateStorage<BeholderAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.beholder.clone(),
        Vec2 { x: 32., y: 32. },
        22,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Enemy {
            xp: 10,
            enemy_type: EnemyType::Beholder,
        })
        .insert(MoveAndShootAI::new(20., 3., 200., 6. / 8., 2.))
        .insert(Velocity::ZERO)
        .insert(Health::new(25))
        .insert(Collider::new_circle(12., Vec2 { x: 70., y: 70. }))
        .insert(make_animation_bundle(
            BeholderAnimation::Flying,
            animations,
            texture_atlas_handle.clone(),
            position,
            1.,
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
}

pub fn spawn_beholder_prince(
    position: Vec3,
    animations: &Res<AnimationStateStorage<BeholderAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.beholder_prince.clone(),
        Vec2 { x: 32., y: 32. },
        22,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Enemy {
            xp: 100,
            enemy_type: EnemyType::BeholderPrince,
        })
        .insert(BeholderPrince)
        .insert(MoveAndShootAI::new(20., 5., 300., 6. / 8., 3.))
        .insert(Velocity::ZERO)
        .insert(Health::new(200))
        .insert(Collider::new_circle(12., Vec2 { x: 100., y: 100. }))
        .insert(make_animation_bundle(
            BeholderAnimation::Flying,
            &animations,
            texture_atlas_handle.clone(),
            position,
            1.,
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
}

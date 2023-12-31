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
    ai::{ChargeShootEvent, FollowPlayerAI, MoveAndShootAI, ShootEvent},
    enemy::{Enemy, EnemyType},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ReaperAnimation {
    Flying,
    Charge,
    End,
}

impl Animation<ReaperAnimation> for ReaperAnimation {
    fn get_states() -> Vec<AnimationStateInfo<ReaperAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(ReaperAnimation::Flying, 4, Duration::from_secs_f32(1. / 8.))
            .add_frames(ReaperAnimation::Charge, 4, Duration::from_secs_f32(1. / 8.))
            .add_single(ReaperAnimation::End)
            .build()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ReaperBladeAnimation {
    Flying,
}

impl Animation<ReaperBladeAnimation> for ReaperBladeAnimation {
    fn get_states() -> Vec<AnimationStateInfo<ReaperBladeAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(
                ReaperBladeAnimation::Flying,
                4,
                Duration::from_secs_f32(1. / 8.),
            )
            .build()
    }
}

#[derive(Component)]
pub struct ReaperBlade {
    pub parent: Entity,
    pub timer: Timer,
}

pub fn reaper_blade_update(
    mut q_blade: Query<(Entity, &mut ReaperBlade)>,
    mut q_ai: Query<&mut MoveAndShootAI, Without<ReaperBlade>>,
    time: Res<Time>,
    mut animate: EventWriter<AnimationStateChangeEvent<ReaperAnimation>>,
    mut commands: Commands,
) {
    for (entity, mut blade) in q_blade.iter_mut() {
        blade.timer.tick(time.delta());

        if blade.timer.just_finished() {
            animate.send(AnimationStateChangeEvent {
                id: blade.parent,
                state_id: ReaperAnimation::Flying,
            });
            commands.entity(entity).despawn();

            if let Ok(mut ai) = q_ai.get_mut(blade.parent) {
                ai.speed = 40.;
            }
        }
    }
}

pub fn reaper_update(
    mut q_reapers: Query<
        (
            Entity,
            &Transform,
            &AnimationController<ReaperAnimation>,
            &mut MoveAndShootAI,
        ),
        Without<Player>,
    >,
    q_player: Query<(Entity, &Transform), With<Player>>,
    mut shoot_ev: EventReader<ShootEvent>,
    mut charge_ev: EventReader<ChargeShootEvent>,
    mut animate: EventWriter<AnimationStateChangeEvent<ReaperAnimation>>,
    beholder_projetile_animations: Res<AnimationStateStorage<ReaperBladeAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    fx: Res<FXChannel>,
    audio: Res<AudioAssets>,
    mut commands: Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.reaper_blade.clone(),
        Vec2 { x: 64., y: 64. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for charge in charge_ev.iter() {
        if let Ok((entity, _, _, _)) = q_reapers.get(charge.entity) {
            animate.send(AnimationStateChangeEvent {
                id: entity,
                state_id: ReaperAnimation::Charge,
            })
        }
    }

    for shoot in shoot_ev.iter() {
        if let Ok((entity, transform, _, mut ai)) = q_reapers.get_mut(shoot.entity) {
            animate.send(AnimationStateChangeEvent {
                id: entity,
                state_id: ReaperAnimation::End,
            });
            let (player_entity, player_transform) = q_player.single();

            fx.play(audio.blade.clone());

            ai.speed = 0.;

            commands
                .spawn(make_animation_bundle(
                    ReaperBladeAnimation::Flying,
                    &beholder_projetile_animations,
                    texture_atlas_handle.clone(),
                    transform.translation,
                    1.2,
                ))
                .insert(ReaperBlade {
                    parent: entity,
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Projectile {
                    dmg: 1,
                    damage_target: DamageTarget::Team(Team::Player),
                    piercing_mode: PiercingMode::All,
                    entities_hit: vec![],
                    is_alive: true,
                })
                .insert(Collider::new_circle(50., transform.translation.truncate()));
        }
    }
}

pub fn spawn_reaper(
    position: Vec3,
    animations: &Res<AnimationStateStorage<ReaperAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.reaper.clone(),
        Vec2 { x: 64., y: 64. },
        9,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Enemy {
            xp: 150,
            enemy_type: EnemyType::Reaper,
        })
        .insert(MoveAndShootAI::new(40., 10., 50., 1. / 2., 2.))
        .insert(Velocity::ZERO)
        .insert(Health::new(300))
        .insert(Collider::new_circle(12., Vec2 { x: 70., y: 70. }))
        .insert(make_animation_bundle(
            ReaperAnimation::Flying,
            animations,
            texture_atlas_handle.clone(),
            position,
            1.,
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
}

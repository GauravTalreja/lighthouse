use std::time::Duration;

use bevy::prelude::*;

use crate::{animation::{AnimationStateStorage, make_animation_bundle, info::AnimationStateInfo, Animation}, loading::TextureAssets, movement::velocity::Velocity, combat::{health::Health, teams::{TeamMember, Team}, healthbar::NeedsHealthBar}, collision::collider::Collider};

use super::{enemy::{Enemy, EnemyType}, ai::FollowPlayerAI};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImpAnimation {
    Flying,
}

impl Animation<ImpAnimation> for ImpAnimation {
    fn get_states() -> Vec<AnimationStateInfo<ImpAnimation>> {
        vec![AnimationStateInfo {
            id: ImpAnimation::Flying,
            start_index: 0,
            frame_count: 4,
            frame_duration: Duration::from_secs_f32(1. / 8.),
        }]
    }
}

pub fn spawn_imp(
    position : Vec3,
    imp_animations: &Res<AnimationStateStorage<ImpAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.imp.clone(),
        Vec2 { x: 32., y: 32. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Enemy {
            xp: 5,
            enemy_type: EnemyType::Imp,
        })
        .insert(FollowPlayerAI{ speed: 15., corrective_force: 1.0 })
        .insert(Velocity::ZERO)
        .insert(Health::new(15))
        .insert(Collider::new_circle(10., position.truncate()))
        .insert(make_animation_bundle(
            ImpAnimation::Flying,
            imp_animations,
            texture_atlas_handle.clone(),
            position,
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
}
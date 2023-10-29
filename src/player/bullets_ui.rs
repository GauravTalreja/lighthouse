use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};


use crate::{
    animation::{
        animation_bundle, AnimationStateChangeEvent,
        AnimationStateStorage, Animation, info::AnimationStateInfo,
    },
    loading::TextureAssets,
};

use super::Player;

#[derive(Component)]
pub struct BulletUISprite {
    index: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BulletUIAnimation {
    Available,
    Unavailable,
}

impl Animation<BulletUIAnimation> for BulletUIAnimation {
    fn get_states() -> Vec<AnimationStateInfo<BulletUIAnimation>> {
        vec![
            AnimationStateInfo {
                id: BulletUIAnimation::Available,
                start_index: 0,
                frames: 1,
                frame_duration: Duration::from_secs_f32(1.),
            },
            AnimationStateInfo {
                id: BulletUIAnimation::Unavailable,
                start_index: 1,
                frames: 1,
                frame_duration: Duration::from_secs_f32(1.),
            },
        ]
    }
}

pub type BulletUIAnimations = AnimationStateStorage<BulletUIAnimation>;

#[derive(Resource)]
pub struct BulletUICount(pub u32);

pub fn manage_bullet_ui_sprites(
    q_player: Query<&Player, Without<BulletUISprite>>,
    mut q_bullets: Query<
        (Entity, &BulletUISprite, &TextureAtlasSprite, &mut Transform),
        Without<Player>,
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut animation_state_change: EventWriter<AnimationStateChangeEvent<BulletUIAnimation>>,
    animations: Res<BulletUIAnimations>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut bullet_count: ResMut<BulletUICount>,
    mut commands: Commands,
) {
    let player = q_player.single();
    let window = q_windows.single();

    while bullet_count.0 < player.max_bullets {
        spawn_bullet_ui_sprite(
            &animations,
            &textures,
            &mut texture_atlases,
            &mut commands,
            bullet_count.0,
        );

        bullet_count.0 += 1;
    }

    for (entity, bullet, atlas, mut transform) in q_bullets.iter_mut() {
        if bullet.index >= player.max_bullets {
            commands.entity(entity).despawn();
            continue;
        }

        if atlas.index == 0 && bullet.index >= player.curr_bullets {
            animation_state_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: BulletUIAnimation::Unavailable,
            })
        } else if atlas.index == 1 && bullet.index < player.curr_bullets {
            animation_state_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: BulletUIAnimation::Available,
            })
        }

        transform.translation = Vec3 {
            x: window.width() / 2. - 40.,
            y: window.height() / 2. - 30. - 20. * (player.max_bullets - 1 - bullet.index) as f32,
            z: 5.,
        }
    }
}

fn spawn_bullet_ui_sprite(
    animations: &Res<BulletUIAnimations>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
    index: u32,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_bullet_ui.clone(),
        Vec2 { x: 16., y: 16. },
        2,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(BulletUISprite { index })
        .insert(animation_bundle(
            BulletUIAnimation::Available,
            animations,
            texture_atlas_handle,
            default(),
        ));
}

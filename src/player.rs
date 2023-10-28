
use std::time::Duration;

use crate::actions::Actions;
use crate::animation::{
    make_animation_bundle, AnimationController, AnimationStateChangeEvent,
    AnimationStateInfo, AppAnimationSetup,
};
use crate::collision::collider::Collider;
use crate::constants::SortingLayers;
use crate::experience::experience_meter::Experience;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

use self::animations::{PlayerAnimationState, PlayerAnimations};
use self::bullets_ui::{manage_bullet_ui_sprites, BulletUIAnimationState, BulletUICount};
use self::reload_ui::{spawn_reload_ui, update_reload_ui, ReloadTimer};
use self::shooting::{shoot, ShootingCooldown};

mod animations;
mod bullets_ui;
mod reload_ui;
mod shooting;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    curr_bullets: usize,
    max_bullets: usize,
    reload_time: f32,
    shoot_time: f32,
    is_reloading: bool,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (
                spawn_player,
                spawn_reload_ui
            ))
            .add_systems(Update, (
                move_player,
                shoot,
                manage_bullet_ui_sprites,
                update_reload_ui,
            ).run_if(in_state(GameState::Playing)))
            .insert_resource(ReloadTimer(Timer::from_seconds(0., TimerMode::Once)))
            .insert_resource(BulletUICount(0))
            .insert_resource(ShootingCooldown(Timer::from_seconds(1.0, TimerMode::Once)))
            .add_animation(vec![
                AnimationStateInfo {
                    id: PlayerAnimationState::Idle,
                    start_index: 0,
                    frames: 2,
                    frame_duration: Duration::from_secs_f32(1. / 2.),
                },
                AnimationStateInfo {
                    id: PlayerAnimationState::Running,
                    start_index: 2,
                    frames: 4,
                    frame_duration: Duration::from_secs_f32(1. / 10.),
                },
            ])
            .add_animation(vec![
                AnimationStateInfo {
                    id: BulletUIAnimationState::Available,
                    start_index: 0,
                    frames: 1,
                    frame_duration: Duration::from_secs_f32(1.),
                },
                AnimationStateInfo {
                    id: BulletUIAnimationState::Unavailable,
                    start_index: 1,
                    frames: 1,
                    frame_duration: Duration::from_secs_f32(1.),
                },
            ]);
    }
}

pub fn spawn_player(
    player_animations: Res<PlayerAnimations>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_hatman.clone(),
        Vec2 { x: 32., y: 32. },
        6,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Player {
            max_bullets: 6,
            curr_bullets: 6,
            reload_time: 1.,
            shoot_time: 0.5,
            is_reloading: false,
        })
        .insert(Collider::new_circle(50., Vec2 { x: 0., y: 0. }))
        .insert(make_animation_bundle(
            PlayerAnimationState::Idle,
            &player_animations,
            texture_atlas_handle,
            Vec3 {
                x: 0.,
                y: 0.,
                z: SortingLayers::Player.into(),
            },
        )).insert(Experience{
            curr_experience: 0,
            level: 0,
            xp_threshold: 10,
            xp_pickup_distance: 6.0,
        });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut animation_change: EventWriter<AnimationStateChangeEvent<PlayerAnimationState>>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut AnimationController<PlayerAnimationState>,
        &mut TextureAtlasSprite,
    )>,
) {
    let (entity, mut player_transform, mut animation_controller, _) =
        player_query.single_mut();

    if actions.player_movement.is_none() {
        if animation_controller.get_state() != PlayerAnimationState::Idle {
            animation_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: PlayerAnimationState::Idle,
            });
        }

        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );

    player_transform.translation += movement;

    if animation_controller.get_state() != PlayerAnimationState::Running {
        animation_change.send(AnimationStateChangeEvent {
            id: entity,
            state_id: PlayerAnimationState::Running,
        });
    }

    if movement.x > 0. && !animation_controller.is_facing_right() {
        animation_controller.set_facing_right(true);
    } else if movement.x < 0. && animation_controller.is_facing_right() {
        animation_controller.set_facing_right(false);
    }
}

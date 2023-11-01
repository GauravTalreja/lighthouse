use bevy::prelude::*;

use crate::{animation::AppAnimationSetup, GameState};

use self::{
    ai::{follow_player, move_and_shoot_ai, ChargeShootEvent, ShootEvent},
    beholder::{beholder_update, BeholderAnimation, BeholderProjectileAnimation},
    enemy::{death_loop, initial_spawn, spread_enemies, EnemyDeathEvent},
    imp::ImpAnimation,
    spawning::{spawn_loop, SpawnInfo},
};

pub mod ai;
pub mod beholder;
pub mod enemy;
pub mod imp;
pub mod spawning;
pub mod zombie;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDeathEvent>()
            .add_systems(
                Update,
                (
                    follow_player,
                    move_and_shoot_ai,
                    death_loop,
                    spread_enemies,
                    spawn_loop,
                    beholder_update,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Playing), initial_spawn)
            .add_animation::<ImpAnimation>()
            .add_animation::<BeholderAnimation>()
            .add_animation::<BeholderProjectileAnimation>()
            .add_event::<ShootEvent>()
            .add_event::<ChargeShootEvent>()
            .insert_resource(SpawnInfo {
                timer: Timer::from_seconds(6., TimerMode::Repeating),
                count: 0,
            });
    }
}

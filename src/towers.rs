use bevy::prelude::*;

pub mod tower;
pub mod turret;
pub mod targeting;
pub mod bullet;
use crate::{towers::tower::tower_trigger, GameState, enemies::enemy::spawn_enemy};

use self::{tower::spawn_tower, bullet::{bullet_move, bullet_collision_check}, turret::follow_tower};

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_trigger.run_if(in_state(GameState::Playing)))
            .add_systems(Update, bullet_move.run_if(in_state(GameState::Playing)))
            .add_systems(Update, bullet_collision_check.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), spawn_tower)
            .add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_systems(Update, follow_tower.run_if(in_state(GameState::Playing)));
    }
}

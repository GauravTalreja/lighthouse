use bevy::prelude::*;


pub mod tower;
pub mod turret;
use crate::{towers::tower::tower_trigger, GameState};

use self::{tower::spawn_tower, turret::follow_mouse};

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_trigger.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), spawn_tower)
            .add_systems(Update, follow_mouse);
    }
}
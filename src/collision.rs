use bevy::prelude::*;

pub mod collider;

use crate::GameState;
use self::collider::{CollisionEvent, collision_tick};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_tick.run_if(in_state(GameState::Playing)))
            .add_event::<CollisionEvent>();
    }
}





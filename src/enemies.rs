use bevy::prelude::*;

use crate::GameState;

use self::enemy::{death_loop, EnemyDeathEvent, follow_mouse};

pub mod enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, death_loop.run_if(in_state(GameState::Playing)))
            .add_event::<EnemyDeathEvent>()
            .add_systems(Update, follow_mouse.run_if(in_state(GameState::Playing)));
    }
}

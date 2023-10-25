use bevy::prelude::*;

use super::tower::Tower;

/*
This is the sprite on top of a tower which turns to shoot enemies
 */

#[derive(Component)]
pub struct Turret {
    pub parent : Entity
}

pub fn follow_tower(
    mut q_turret: Query<(Entity, &mut Turret, &mut Transform)>,
    q_tower: Query<(Entity, &Tower), Without<Turret>>
) {
    for (_, turret, mut transform) in q_turret.iter_mut() {
        if let Ok((_, tower)) = q_tower.get(turret.parent) {
            transform.rotation = Quat::from_rotation_z(tower.rotation.angle);
        }
    }
}
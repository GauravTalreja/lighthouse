use bevy::prelude::*;

use crate::enemies::enemy::{Enemy, Health};
use crate::collision::collider::CollisionEvent;

#[derive(Component)]
pub struct Bullet {
    pub angle : Vec2,
    pub velocity : f32,
    pub dmg : Health,
}

pub fn bullet_collision_check(
    mut q_bullets : Query<&mut Bullet>,
    mut q_enemies : Query<&mut Enemy>,
    mut ev_collision : EventReader<CollisionEvent>,
    mut commands : Commands
) {
    for collision in ev_collision.iter() {
        if let (Ok(bullet), Ok(enemy)) = (q_bullets.get_mut(collision.entity_a), q_enemies.get_mut(collision.entity_b)) {
            bullet_collision(
                collision.entity_a, 
                bullet,  
                collision.entity_b, 
                enemy, 
                &mut commands
            );
        }
        if let (Ok(bullet), Ok(enemy)) = (q_bullets.get_mut(collision.entity_b), q_enemies.get_mut(collision.entity_a)) {
            bullet_collision(
                collision.entity_b, bullet,  
                collision.entity_a, 
                enemy, 
                &mut commands
            );
        }
    }
}

fn bullet_collision(
    bullet_entity : Entity,
    bullet : Mut<Bullet>,
    _enemy_entity : Entity,
    mut enemy : Mut<Enemy>,
    commands : &mut Commands,
) {
    commands.entity(bullet_entity).despawn();
    enemy.take_damage(bullet.dmg);
}

pub fn bullet_move(
    mut q_bullets : Query<(Entity, &Bullet, &mut Transform)>,
    time : Res<Time>,
) {
    for (_, bullet, mut transform) in q_bullets.iter_mut() {
        let diff = bullet.angle * time.delta_seconds() * bullet.velocity;
        transform.translation += Vec3{
            x: diff.x,
            y: diff.y,
            z: 0.
        };
    }
}
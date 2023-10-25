use crate::collision::collider::Collider;
use crate::cooldown::Cooldown;
use crate::enemies::enemy::Enemy;
use crate::loading::TextureAssets;
use crate::radians::Radian;
use crate::towers::turret::Turret;
use bevy::prelude::*;


use std::f32::consts::PI;
use std::time::Duration;

use super::bullet::Bullet;
use super::targeting::Targeting;

pub struct TowerStats {
    pub range: f32,
    pub cooldown: Duration,
    pub targeting : Targeting,
    pub rotation_speed : Radian,
}

impl TowerStats {
    pub fn fire_rate(&self) -> f32 {
        1. / self.cooldown.as_secs_f32()
    }
}

#[derive(Component)]
pub struct Tower {
    pub stats: TowerStats,
    pub rotation: Radian, // in radians
}

pub fn tower_trigger(
    mut towers: Query<(Entity, &mut Tower, &mut Transform, &mut Cooldown)>,
    mut enemies: Query<(Entity, &mut Enemy, &mut Transform), Without<Tower>>,
    mut commands : Commands,
    textures: Res<TextureAssets>,
    time : Res<Time>,
) {

    for (_, mut tower, tower_transform, mut tower_cooldown) in towers.iter_mut() {
        let mut possible_targets = vec![];
        for (enemys_entity, enemy, enemy_transform) in enemies.iter_mut() {
            let distance_to_enemy = tower_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate()); 
            if distance_to_enemy <= tower.stats.range {
                possible_targets.push((enemys_entity, enemy, enemy_transform));
            }
        }
 
        let target = tower.stats.targeting.find_best_target(&possible_targets);
        if let Some((_, _, target_transform)) = target {
            let direction = target_transform.translation.truncate() - tower_transform.translation.truncate();

            // obtain angle to target with respect to x-axis.
            let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.).normalize_to_half();

            let angle_diff = (tower.rotation - angle_to_target).normalize_to_half();
            let allowed_rotation = tower.stats.rotation_speed * time.delta().as_secs_f32();
            

            if angle_diff.abs().angle > allowed_rotation.angle {
                let multiplier = match angle_diff.angle > 0. {
                    true => -1.,
                    false => 1.,
                };

                let rotation = allowed_rotation * multiplier;
                tower.rotation = (tower.rotation + rotation).normalize()
            } else {
                tower.rotation = angle_to_target;

                if !tower_cooldown.is_ready() {
                    continue;
                }

                
                let direction_vec = Vec2{
                    x: -angle_to_target.angle.sin(),
                    y: angle_to_target.angle.cos(),
                };
                let bullet_translation = tower_transform.translation + Vec3{
                    x: direction_vec.x,
                    y: direction_vec.y,
                    z: 0.
                } * 30. + Vec3::Z * 5.;

                // Shoot!
                commands.spawn(SpriteBundle{
                    texture: textures.texture_bullet.clone(),
                    transform: Transform {
                        translation: bullet_translation,
                        scale: Vec3{ x: 2., y: 2., z: 1.},
                        rotation: Quat::IDENTITY,
                    },
                    ..Default::default()
                })
                    .insert(Bullet{
                        angle: direction_vec,
                        velocity: 600.,
                        dmg: 1,
                    })
                    .insert(Collider::new_circle(
                        5., 
                        bullet_translation.truncate()
                    ));
                tower_cooldown.time_remaining += tower.stats.cooldown;
            }
            
        }

        
    }
}

pub fn spawn_tower(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_tower.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                rotation: Quat::IDENTITY,
                scale: 2.
                    * Vec3 {
                        x: 1.,
                        y: 1.,
                        z: 1.,
                    },
            },
            ..Default::default()
        })
        // Collider
        .insert(Collider::new_rect(
            Vec2{x:1000., y: 1000.}, 
            Vec2 { x: 0., y: 0. }
        ))
        // Tower
        .insert(Tower {
            stats: TowerStats {
                range: 200.,
                cooldown: Duration::from_secs_f32(1.),
                targeting: Targeting::First,
                rotation_speed: Radian { angle: PI * 2. }
            },
            rotation: Radian::ZERO,
        })
        .insert(Cooldown{
            time_remaining: Duration::ZERO,
        })
        .with_children(|parent| {
            let parent_entity = parent.parent_entity();
            parent
                .spawn(SpriteBundle {
                    texture: textures.texture_turret.clone(),
                    transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                    ..Default::default()
                })
                .insert(Turret{ parent: parent_entity });
        });
}

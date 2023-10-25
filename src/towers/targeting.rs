use bevy::prelude::*;

use crate::enemies::enemy::Enemy;

pub enum Targeting {
    First,
    Last,
    MostHealth,
    LeastHealth
}

impl Targeting {
    pub fn find_best_target<'a>(&self, enemies : &'a Vec<(Entity, Mut<'a, Enemy>, Mut<'a, Transform>)>) -> Option<&'a (Entity, Mut<'a, Enemy>, Mut<'a, Transform>)> {
        if enemies.is_empty() {
            None
        } else {
            match self {
                Targeting::First => {
                    Some(enemies.iter().max_by_key(|(_, e, _)| (e.track_progress * 100000.) as i32).unwrap())
                },
                Targeting::Last => {
                    Some(enemies.iter().min_by_key(|(_, e, _)| (e.track_progress * 100000.) as i32).unwrap())
                },
                Targeting::MostHealth => {
                    Some(enemies.iter().max_by_key(|(_, e, _)| (e.health) as i32).unwrap())
                },
                Targeting::LeastHealth => {
                    Some(enemies.iter().min_by_key(|(_, e, _)| (e.health) as i32).unwrap())
                },
            }
        }
    }
}
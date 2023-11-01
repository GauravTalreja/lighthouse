use bevy::prelude::*;

#[derive(Component)]
pub struct Experience {
    pub curr_experience: u32,
    pub level: u32,
    pub threshold: u32,
    pub pick_distance: f32,
}

#[derive(Event)]
pub struct LevelUpEvent {
    pub new_level: u32,
}

pub fn experience_update(
    mut q_xp: Query<&mut Experience>,
    mut level_up_ev: EventWriter<LevelUpEvent>,
) {
    let mut xp = q_xp.single_mut();

    if xp.curr_experience >= xp.threshold {
        xp.curr_experience -= xp.threshold;
        xp.level += 1;

        level_up_ev.send(LevelUpEvent {
            new_level: xp.level,
        });
    }
}

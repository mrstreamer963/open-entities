use bevy_ecs::prelude::Component;

/// Component: Velocity of an entity
#[derive(Component, Clone, Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

use bevy_ecs::prelude::Component;

/// Component: Position of an entity
#[derive(Component, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

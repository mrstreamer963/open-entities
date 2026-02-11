use bevy_ecs::prelude::Component;

/// Component: Position of an entity
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

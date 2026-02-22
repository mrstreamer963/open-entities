use bevy_ecs::prelude::Component;

/// Marker component: Unit
/// Used to identify entities as units in the ECS system
#[derive(Component, Clone, Debug)]
pub struct Unit;

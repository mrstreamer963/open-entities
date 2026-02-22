use bevy_ecs::prelude::Component;

/// Marker component: Vehicle
/// Used to identify entities as vehicles in the ECS system
#[derive(Component, Clone, Debug)]
pub struct Vehicle;

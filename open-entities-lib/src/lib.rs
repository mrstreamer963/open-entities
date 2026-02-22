//! # OpenEntities
//!
//! A library for working with entities using the **bevy_ecs** framework.
//!
//! This example demonstrates the basic concepts of ECS:
//! - **Components**: Data attached to entities (Position, Velocity)
//! - **Systems**: Functions that operate on components
//! - **Entities**: Unique objects in the world
//!
//! # Examples
//!
//! ```rust
//! use bevy_ecs::prelude::World;
//! use open_entities::setup_app;
//!
//! fn main() {
//!     let mut world = World::new();
//!     setup_app(&mut world);
//!     // Run systems manually or use a schedule
//! }
//! ```

pub mod components;
pub mod systems;

pub use components::{Position, Unit, Vehicle, Velocity};
pub use components::{
    spawn_from_yaml, spawn_yaml_entities, YamlComponent, YamlEntity, YamlEntityList,
};
pub use systems::{move_system, print_position_system, setup_app};

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::{Entity, World};

    #[test]
    fn test_components_compile() {
        // Test that components can be instantiated
        let _pos = Position { x: 0.0, y: 0.0 };
        let _vel = Velocity { vx: 1.0, vy: 2.0 };
        let _unit = Unit;
        let _vehicle = Vehicle;
    }

    #[test]
    fn test_spawn_entity_and_query() {
        let mut world = World::new();

        // Spawn an entity with both Position and Velocity
        let entity = {
            world
                .spawn((Position { x: 5.0, y: 5.0 }, Velocity { vx: 2.0, vy: 3.0 }))
                .id()
        };

        // Query for entities with Velocity
        {
            let mut query = world.query::<&Velocity>();
            let velocities: Vec<_> = query.iter(&world).collect();
            assert_eq!(velocities.len(), 1);
        }

        // Query for entities with Position but without Velocity
        {
            let mut query = world.query::<(&Position, Entity)>();
            let positions: Vec<_> = query
                .iter(&world)
                .filter(|(_, entity)| world.get::<Velocity>(*entity).is_none())
                .collect();
            assert_eq!(positions.len(), 0);
        }

        // Query for specific entity by ID
        {
            let pos = world.get::<Position>(entity).unwrap();
            assert_eq!(pos.x, 5.0);
            assert_eq!(pos.y, 5.0);
        }
    }
}

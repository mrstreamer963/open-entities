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
//! use bevy_app::App;
//! use open_entities::setup_app;
//!
//! fn main() {
//!     let mut app = App::new();
//!     setup_app(&mut app);
//!     app.run();
//! }
//! ```

pub mod components;
pub mod systems;

pub use components::{Position, Velocity};
pub use systems::{move_system, print_position_system, setup_app};

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::prelude::Entity;
    use super::*;

    #[test]
    fn test_components_compile() {
        // Test that components can be instantiated
        let _pos = Position { x: 0.0, y: 0.0 };
        let _vel = Velocity { vx: 1.0, vy: 2.0 };
    }

    #[test]
    fn test_spawn_entity_and_query() {
        let mut app = App::new();

        // Spawn an entity with both Position and Velocity
        let entity = {
            let world = app.world_mut();
            world
                .spawn((Position { x: 5.0, y: 5.0 }, Velocity { vx: 2.0, vy: 3.0 }))
                .id()
        };

        // Query for entities with Velocity
        {
            let mut query = app.world_mut().query::<&Velocity>();
            let velocities: Vec<_> = query.iter(&app.world()).collect();
            assert_eq!(velocities.len(), 1);
        }

        // Query for entities with Position but without Velocity
        {
            let mut query = app.world_mut().query::<(&Position, Entity)>();
            let positions: Vec<_> = query
                .iter(&app.world())
                .filter(|(_, entity)| app.world().get::<Velocity>(*entity).is_none())
                .collect();
            assert_eq!(positions.len(), 0);
        }

        // Query for specific entity by ID
        {
            let pos = app.world().get::<Position>(entity).unwrap();
            assert_eq!(pos.x, 5.0);
            assert_eq!(pos.y, 5.0);
        }
    }
}

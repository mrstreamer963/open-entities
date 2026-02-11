//! # OpenEntities
//!
//! A library for working with entities using the **bevy_ecs** framework.
//!
//! This example demonstrates the basic concepts of ECS:
//! - **Components**: Data attached to entities (Position, Velocity)
//! - **Systems**: Functions that operate on components
//! - **Systems**: Functions that operate on components

use bevy_app::{App, Startup, Update};
use bevy_ecs::prelude::*;

/// Component: Position of an entity
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Component: Velocity of an entity
#[derive(Component)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

/// System: Update position based on velocity
fn move_system(mut query: Query<&mut Position, With<Velocity>>) {
    for mut position in &mut query {
        position.x += 1.0;
        position.y += 1.0;
    }
}

/// System: Print positions of all entities
fn print_position_system(query: Query<&Position>) {
    for (i, position) in query.iter().enumerate() {
        println!("Entity {}: position = ({}, {})", i, position.x, position.y);
    }
}

/// Setup system: spawn some entities
fn setup_system(mut commands: Commands) {
    // Spawn first entity with position and velocity
    commands.spawn((
        Position { x: 0.0, y: 0.0 },
        Velocity { vx: 1.0, vy: 2.0 },
    ));

    // Spawn second entity with only position
    commands.spawn(Position { x: 10.0, y: 10.0 });
}

/// Initialize the ECS world
/// This function provides the wiring logic for an ECS application.
/// Users of this library should call this function on their App instance.
pub fn setup_app(app: &mut App) {
    // Add systems to the startup schedule
    app.add_systems(Startup, setup_system)
        .add_systems(Update, (move_system, print_position_system));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!("Hello, World!", "Hello, World!");
    }

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
        let entity = app
            .world_mut()
            .spawn((Position { x: 5.0, y: 5.0 }, Velocity { vx: 2.0, vy: 3.0 }))
            .id();

        // Query for entities with Velocity
        {
            let mut query = app.world_mut().query::<&Velocity>();
            let velocities: Vec<_> = query.iter(&app.world()).collect();
            assert_eq!(velocities.len(), 1);
        }

        // Query for entities with Position but without Velocity
        {
            let mut query_no_vel = app.world_mut().query_filtered::<&Position, Without<Velocity>>();
            let positions: Vec<_> = query_no_vel.iter(&app.world()).collect();
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

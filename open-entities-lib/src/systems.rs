use crate::components::{Position, Velocity};
use bevy_ecs::prelude::*;

/// System: Update position based on velocity
pub fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.vx;
        pos.y += vel.vy;
    }
}

/// System: Print positions of all entities
pub fn print_position_system(query: Query<&Position>) {
    for (i, position) in query.iter().enumerate() {
        println!("Entity {}: position = ({}, {})", i, position.x, position.y);
    }
}

/// Setup system: spawn some entities
fn setup_system(mut commands: Commands) {
    // Spawn first entity with position and velocity
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { vx: 1.0, vy: 2.0 }));

    // Spawn second entity with only position
    commands.spawn(Position { x: 10.0, y: 10.0 });
}

/// Initialize the ECS world with a schedule
/// This function provides the wiring logic for an ECS application.
/// Users of this library should call this function on their World instance.
pub fn setup_app(world: &mut World) {
    // Create a new schedule
    let mut schedule = Schedule::default();
    
    // Add startup systems
    schedule.add_systems(setup_system);
    
    // Add update systems
    schedule.add_systems((move_system, print_position_system));
    
    // Run the schedule once to execute startup systems
    schedule.run(world);
}

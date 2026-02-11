use crate::components::{Position, Velocity};
use bevy_app::{App, Startup, Update};
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

/// Initialize the ECS world
/// This function provides the wiring logic for an ECS application.
/// Users of this library should call this function on their App instance.
pub fn setup_app(app: &mut App) {
    // Add systems to the startup schedule
    app.add_systems(Startup, setup_system)
        .add_systems(Update, (move_system, print_position_system));
}

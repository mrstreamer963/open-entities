# AGENTS.md - OpenEntities Project Guide

## Project Overview

OpenEntities is a Rust library for working with entities using the **bevy_ecs** framework (version 0.14). It demonstrates core ECS (Entity Component System) concepts and provides reusable building blocks for ECS-based applications.

## Essential Commands

| Command | Purpose |
|---------|---------|
| `cargo check` | Verify code compiles (fast, no output) |
| `cargo build` | Build the library (produces `target/debug/libopen_entities.rlib`) |
| `cargo test` | Run all tests |
| `cargo doc --open` | Build and open documentation |
| `cargo fmt` | Format code according to Rust style |
| `cargo clippy` | Run linter for code quality checks |

## Code Organization

### Directory Structure
```
open-entities/
├── src/
│   └── lib.rs           # Main library code (ECS components and systems)
├── Cargo.toml           # Dependencies: bevy_ecs 0.14, bevy_app 0.14
└── AGENTS.md            # This file
```

### File: `src/lib.rs`

The library contains:

1. **Components** (data attached to entities):
   - `Position`: 2D coordinates (x, y)
   - `Velocity`: 2D velocity vector (vx, vy)

2. **Systems** (functions operating on components):
   - `move_system`: Updates position based on velocity
   - `print_position_system`: Logs entity positions
   - `setup_system`: Spawns initial entities

3. **Core Function**:
   - `setup_app(app: &mut App)`: Configures an `App` with all systems

## Code Patterns

### Naming Conventions
- **Types**: PascalCase (e.g., `Position`, `Velocity`)
- **Functions/Methods**: snake_case (e.g., `move_system`, `setup_app`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Lifetime parameters**: lowercase single letters (e.g., `'a`)
- **Generic parameters**: uppercase single letters (e.g., `T`, `E`)

### Component Patterns
- Components use `#[derive(Component)]`
- Public fields for data access
- Related data grouped into separate component types

### System Patterns
- Systems take parameters via `Query<T>` for component access
- Use `With<T>` or `Without<T>` constraints for filtering
- Mutability indicated by `&mut` references

### Schedule Configuration
```rust
app.add_systems(Startup, setup_system)        // Runs once at startup
   .add_systems(Update, (move_system, print_position_system));  // Runs every frame
```

## Testing

- Tests use standard `#[cfg(test)]` module
- Tests run with `cargo test`
- Current tests verify:
  - Component instantiation
  - Entity spawning with `Commands::spawn()`
  - Query creation and iteration
  - Filtering with `With<T>` and `Without<T>`
  - Entity lookup by ID using `World::get()`

### Test Patterns
```rust
#[test]
fn test_spawn_entity_and_query() {
    let mut app = App::new();
    
    // Spawn entity with multiple components
    let entity = app
        .world_mut()
        .spawn((Position { x: 5.0, y: 5.0 }, Velocity { vx: 2.0, vy: 3.0 }))
        .id();
    
    // Query with filter
    let mut query = app.world_mut().query::<&Velocity>();
    let velocities: Vec<_> = query.iter(&app.world()).collect();
    
    // Entity lookup by ID
    let pos = app.world().get::<Position>(entity).unwrap();
}
```

## Gotchas and Notes

1. **bevy_ecs 0.14 API changes**: 
   - `App` is in `bevy_app` crate (import: `use bevy_app::App`)
   - `Startup` and `Update` are in `bevy_app`, not `bevy_ecs::schedule`

2. **World borrowing rules**:
   - You can only have one mutable borrow of `World` at a time
   - In tests, store `world_mut()` in a variable once and reuse it
   - In systems, bevy handles borrowing automatically

3. **Query types in tests**:
   - Use `world.query::<T>()` for basic queries
   - Use `world.query_filtered::<T, F>()` for filtered queries
   - Call `.iter(&world)` to iterate results

4. **Library design**: This is a library, not a binary. Users need to:
   - Create their own `App`
   - Call `setup_app(&mut app)` to configure it
   - Run the app with `app.run()` (in a binary context)

5. **Edition**: Uses Rust 2021 edition

## Adding New Features

1. Create a new component with `#[derive(Component)]`
2. Implement a system function with appropriate `Query` parameters
3. Add the system to `setup_app()` using `app.add_systems()`
4. Run `cargo test` to verify
5. Run `cargo fmt` before committing

## Dependencies

- `bevy_ecs = "0.14"` - ECS framework
- `bevy_app = "0.14"` - Application framework (provides `App`, `Startup`, `Update`)

# OpenEntities

A library for working with entities using the **bevy_ecs** framework.

## Features

- **Components**: Data attached to entities
  - `Position`: 2D coordinates (x, y)
  - `Velocity`: 2D velocity vector (vx, vy)
- **Systems**: Functions that operate on components
  - `move_system`: Updates position based on velocity
  - `print_position_system`: Logs entity positions
  - `setup_system`: Spawns initial entities

## Usage

```rust
use bevy_app::App;
use open_entities::setup_app;

fn main() {
    let mut app = App::new();
    setup_app(&mut app);
    app.run();
}
```

## Running Tests

```bash
cargo test
```

## License

MIT

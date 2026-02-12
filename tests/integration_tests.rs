use bevy_app::App;
use open_entities::Position;
use open_entities::Velocity;
use open_entities::setup_app;

#[test]
fn test_components_compile() {
    let _pos = Position { x: 0.0, y: 0.0 };
    let _vel = Velocity { vx: 1.0, vy: 2.0 };
}

#[test]
fn test_move_system() {
    let mut app = App::new();
    setup_app(&mut app);

    app.update();

    let mut query = app.world_mut().query::<(&Position, &Velocity)>();
    let entities: Vec<_> = query.iter(&app.world()).collect();

    // Only entities with Velocity are in this query
    assert_eq!(entities.len(), 1);

    let (pos1, vel1) = &entities[0];
    assert_eq!(pos1.x, 1.0);
    assert_eq!(pos1.y, 2.0);
    let _vel1 = vel1;

    // Check entity without velocity still exists at original position
    let mut query_no_vel = app.world_mut().query::<&Position>();
    let positions: Vec<_> = query_no_vel.iter(&app.world()).collect();
    assert_eq!(positions.len(), 2);
    assert_eq!(positions[1].x, 10.0);
    assert_eq!(positions[1].y, 10.0);
}

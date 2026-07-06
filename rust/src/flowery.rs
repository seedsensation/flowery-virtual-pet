use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

use super::window_management::{get_window_shape, Shape};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Flowery {
    primary_window: Shape,
    #[var]
    /// The speed at which my boy moves
    velocity: Vector2,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Flowery {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!");

        Self {
            primary_window: Shape::empty(),
            velocity: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {}
    fn process(&mut self, _delta: f64) {
        get_window_shape().map(|shape| self.primary_window = shape);
    }
}

#[godot_api]
impl Flowery {
    #[func]
    /// Move Flowery, and handle collision
    pub fn move_and_slide(&mut self) {
        godot_print!("{}", self.velocity);
        let mut window = self.base().get_window().unwrap();
        //let mut display_server = DisplayServer::singleton();

        let window_position = window.get_position();
        let (x, y) = self.velocity.to_tuple();
        //display_server.window_set_position(window_position + Vector2i::new(x as i32, y as i32));
        window.set_position(window_position + Vector2i::new(x as i32, y as i32));
    }

    #[func]
    /// Move Flowery to a specific position
    pub fn move_to(&mut self, location: Vector2i) {
        let mut window = self.base().get_window().unwrap();
        window.set_position(location);
    }
}

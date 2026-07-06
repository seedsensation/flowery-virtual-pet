use crate::vector_cast::VectorCast;
use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

use super::window_management::{get_window_shape, Shape};

const DISTANCE_TO_FALL: i32 = 100;

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
        let shape1 = Shape {
            pos: Vector2i { x: 0, y: 0 },
            size: Vector2i { x: 1, y: 1 },
        };
        let shape2 = Shape {
            pos: Vector2i { x: 0, y: 0 },
            size: Vector2i { x: 1, y: 2 },
        };
        godot_print!("{}", collision_check(&shape1, &shape2));

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
    /// Get Flowery's current sprite's shape
    pub fn get_shape(&self) -> Shape {
        match self.base().find_child("Sprite") {
            Some(sprite) if sprite.is_class("AnimatedSprite2D") => {
                let sprite = sprite.cast::<godot::classes::AnimatedSprite2D>();
                let animation = sprite.get_animation().to_string();
                let texture = sprite
                    .get_sprite_frames()
                    .unwrap()
                    .get_frame_texture(&animation, 0)
                    .unwrap();
                Shape {
                    pos: self.base().get_window().unwrap().get_position(),
                    size: (texture.get_size() * sprite.get_scale()).to_int_vector(),
                }
            }
            _ => Shape::empty(),
        }
    }

    #[func]
    /// Move Flowery, and handle collision
    pub fn move_and_slide(&mut self) {
        let mut window = self.base().get_window().unwrap();
        //let mut display_server = DisplayServer::singleton();

        let window_position = window.get_position();
        //display_server.window_set_position(window_position + Vector2i::new(x as i32, y as i32));
        if self.test_collision(self.velocity.to_int_vector()) {
            godot_print!("COLLIDING WITH ACTIVE WINDOW");
        } else {
            godot_print!("NOT COLLIDING");
            window.set_position(window_position + self.velocity.to_int_vector());
        }
    }

    #[func]
    /// Move Flowery to a specific position
    pub fn move_to(&mut self, location: Vector2i) {
        let mut window = self.base().get_window().unwrap();
        window.set_position(location);
    }

    #[func]
    /// Returns true if Flowery is currently colliding with the active window
    pub fn check_collision(&self) -> bool {
        collision_check(&self.get_shape(), &self.primary_window)
    }

    #[func]
    pub fn will_collide(&self) -> bool {
        self.test_collision(self.velocity)
    }

    /// Returns true if the vector of movement would make Flowery collide with the active window
    pub fn test_collision<T: VectorCast>(&self, vector: T) -> bool {
        let mut new_shape = self.get_shape();
        new_shape.pos += vector.to_int_vector();

        collision_check(&new_shape, &self.primary_window)
    }

    fn collision_sides(&self) -> Vector2 {
        let mut vector = Vector2::ZERO;
        for (x, y) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            let new_vec = Vector2::new(x, y);
            if self.test_collision(new_vec) {
                vector += new_vec;
            }
        }
        vector.normalized()
    }

    #[signal]
    fn falling_from_height();

    #[signal]
    fn trapped();
}

fn collision_check(a: &Shape, b: &Shape) -> bool {
    // a.left < b.right &&
    // a.right > b.left &&
    // a.top > b.bottom &&
    // a.bottom < b.top
    // check if colliding with active window
    a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
}

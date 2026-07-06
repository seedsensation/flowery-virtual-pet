use crate::vector_cast::VectorCast;
use godot::classes::DisplayServer;
use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

use super::window_management::{get_window_shape, Shape};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Flowery {
    primary_window: Shape,

    #[var]
    idle_timer: f64,
    #[var]
    ignore_collision: bool,
    #[var]
    /// The speed at which my boy moves
    velocity: Vector2,
    #[var]
    out_of_bounds: bool,
    #[var]
    /// How much he accelerates by
    acceleration: Vector2,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Flowery {
    fn init(base: Base<Node2D>) -> Self {
        let shape1 = Shape {
            pos: Vector2i { x: 0, y: 0 },
            size: Vector2i { x: 1, y: 1 },
        };
        let shape2 = Shape {
            pos: Vector2i { x: 0, y: 0 },
            size: Vector2i { x: 1, y: 2 },
        };

        Self {
            primary_window: Shape::empty(),
            ignore_collision: false,
            idle_timer: 0.0,
            velocity: Vector2::new(1.0, 0.0),
            out_of_bounds: false,
            acceleration: Vector2::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.velocity += self.acceleration * delta as f32;
    }
    fn process(&mut self, delta: f64) {
        self.idle_timer += delta;
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
        let display_server = DisplayServer::singleton();
        let screen_size = display_server.screen_get_size();

        let window_position = window.get_position();
        //display_server.window_set_position(window_position + Vector2i::new(x as i32, y as i32));

        let mut shape = self.get_shape();
        shape.pos += self.velocity.to_int_vector();
        if !shapes_overlap(
            &shape,
            &(Shape {
                pos: Vector2i {
                    x: 0,
                    y: screen_size.y,
                },
                size: Vector2i {
                    x: screen_size.x,
                    y: 1,
                },
            }),
        ) {
            window.set_position(window_position + self.velocity.to_int_vector());
        }
    }

    #[func]
    /// Move Flowery to a specific position
    pub fn move_to(&mut self, location: Vector2i) {
        let mut window = self.base().get_window().unwrap();
        window.set_position(location);
    }

    #[signal]
    fn screen_border_collision();

    #[signal]
    fn window_collision();
}

fn shapes_overlap(a: &Shape, b: &Shape) -> bool {
    // a.left < b.right &&
    // a.right > b.left &&
    // a.top > b.bottom &&
    // a.bottom < b.top
    // check if colliding with active window
    a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
}

// Set clippy lints - I want this code to be as solid as possible,
// so i want clippy to be a pain
#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
use crate::vector_cast::VectorCast;
use godot::classes::{AnimatedSprite2D, Area2D, DisplayServer};
use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MovableWindow {
    #[var(pub)]
    ignore_collision: bool,
    #[export]
    #[var(pub)]
    /// The speed at which my boy moves
    velocity: Vector2,
    #[export]
    #[var(pub)]
    /// How much he accelerates by
    acceleration: Vector2,
    #[var(pub)]
    sprite: Option<Gd<AnimatedSprite2D>>,
    #[var(pub)]
    area: Option<Gd<Area2D>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MovableWindow {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            sprite: None,
            area: None,
            ignore_collision: false,
            velocity: Vector2::new(1.0, 0.0),
            acceleration: Vector2::ZERO,
            base,
        }
    }

    //fn physics_process(&mut self, delta: f64) {
    //    self.velocity += self.acceleration * delta as f32;
    //}
    //fn process(&mut self, delta: f64) {}
}

#[godot_api]
impl MovableWindow {
    #[func]
    /// Get Flowery's current sprite's shape
    pub fn get_shape(&self) -> Rect2 {
        let sprite = self.sprite.as_ref().unwrap();
        let animation = sprite.get_animation().to_string();
        let texture = sprite
            .get_sprite_frames()
            .unwrap()
            .get_frame_texture(&animation, 0)
            .unwrap();
        Rect2::new(
            self.base()
                .get_window()
                .unwrap()
                .get_position()
                .to_flt_vector(),
            texture.get_size() * sprite.get_scale(),
        )
    }

    #[func]
    #[allow(unused)]
    /// Sets the sprite's offset.
    ///
    /// This is a lot more difficult than you'd expect, so I'm going to go through it step-by-step.
    pub fn set_offset(&mut self, offset: Vector2) {}

    #[func]
    pub fn readjust_window_size(&mut self) {
        let scale = self.sprite.as_ref().unwrap().get_scale();
        let sprite = self.sprite.as_mut().unwrap();
        let texture = sprite
            .get_sprite_frames()
            .unwrap()
            .get_frame_texture(&sprite.get_animation().to_string(), 0)
            .unwrap();

        DisplayServer::singleton().window_set_size(Vector2i::new(
            texture.get_width() * scale.cast_int().x,
            texture.get_height() * scale.cast_int().y,
        ));
    }

    #[func]
    pub fn get_position(&self) -> Vector2 {
        self.base()
            .get_window()
            .unwrap()
            .get_position()
            .to_flt_vector()
    }

    #[func]
    /// Move Flowery, and handle collision
    pub fn move_and_slide(&mut self, delta: f64) {
        if self.velocity == Vector2::ZERO && self.acceleration == Vector2::ZERO {
            return;
        }
        let mut window = self.base().get_window().unwrap();
        let display_server = DisplayServer::singleton();

        let window_position = window.get_position();
        //display_server.window_set_position(window_position + Vector2i::new(x as i32, y as i32));

        let character_rect = self.get_shape();
        let usable_rect = display_server.screen_get_usable_rect();

        if !usable_rect.encloses(character_rect.cast_int()) {
            let (up, right, down, left) = (
                self.touching_top_side(),
                self.touching_right_side(),
                self.touching_bottom_side(),
                self.touching_left_side(),
            );
            self.signals()
                .screen_border_collision()
                .emit(up, right, down, left);
        }
        window.set_position(window_position + (self.velocity * delta as f32).to_int_vector());
    }

    #[func]
    #[inline]
    pub fn touching_left_side(&self) -> bool {
        self.get_shape().position.x <= 0f32
    }

    #[func]
    #[inline]
    pub fn touching_right_side(&self) -> bool {
        self.get_shape().end().x
            >= DisplayServer::singleton().screen_get_usable_rect().end().x as f32
    }

    #[func]
    #[inline]
    pub fn touching_top_side(&self) -> bool {
        self.get_shape().position.y <= 0f32
    }

    #[func]
    #[inline]
    pub fn touching_bottom_side(&self) -> bool {
        self.get_shape().end().y
            >= DisplayServer::singleton().screen_get_usable_rect().end().y as f32
    }

    #[func]
    /// Move Flowery to a specific position
    pub fn move_to(&mut self, location: Vector2i) {
        let mut window = self.base().get_window().unwrap();
        window.set_position(location);
    }

    #[signal]
    fn screen_border_collision(up: bool, right: bool, down: bool, left: bool);

    #[signal]
    fn window_collision();
}

//fn shapes_overlap(a: &Shape, b: &Shape) -> bool {
//    // a.left < b.right &&
//    // a.right > b.left &&
//    // a.top > b.bottom &&
//    // a.bottom < b.top
//    // check if colliding with active window
//    a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
//}

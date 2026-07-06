use crate::vector_cast::VectorCast;
use godot::classes::DisplayServer;
use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Flowery {
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
        Self {
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
    }
}

#[godot_api]
impl Flowery {
    /// Get Flowery's current sprite's shape
    pub fn get_shape(&self) -> Rect2 {
        match self.base().find_child("Sprite") {
            Some(sprite) if sprite.is_class("AnimatedSprite2D") => {
                let sprite = sprite.cast::<godot::classes::AnimatedSprite2D>();
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
            _ => Rect2::new(Vector2::ZERO, Vector2::ZERO),
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

        let character_rect = self.get_shape();
        let usable_rect = display_server.screen_get_usable_rect();

        if usable_rect.encloses(character_rect.cast_int()) {
            window.set_position(window_position + self.velocity.to_int_vector());
        } else {
            godot_print!("Hit border");
            self.signals().screen_border_collision().emit();
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

//fn shapes_overlap(a: &Shape, b: &Shape) -> bool {
//    // a.left < b.right &&
//    // a.right > b.left &&
//    // a.top > b.bottom &&
//    // a.bottom < b.top
//    // check if colliding with active window
//    a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
//}

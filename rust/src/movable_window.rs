use crate::vector_cast::VectorCast;
use godot::classes::{AnimatedSprite2D, Area2D, DisplayServer};
use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct MovableWindow {
    #[var]
    ignore_collision: bool,

    #[var]
    window_offset: Vector2,
    #[export]
    /// The speed at which my boy moves
    velocity: Vector2,
    #[export]
    /// How much he accelerates by
    acceleration: Vector2,
    #[var]
    sprite: Option<Gd<AnimatedSprite2D>>,
    #[var]
    area: Option<Gd<Area2D>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MovableWindow {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            sprite: None,
            area: None,
            window_offset: Vector2::new(0.0, 0.0),
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
    pub fn readjust_with_additional_sprite(&mut self, other_sprite: Gd<AnimatedSprite2D>) {
        let offset = self.sprite.as_ref().unwrap().get_offset();
        let scale = self.sprite.as_ref().unwrap().get_scale();
        let sprite = self.sprite.as_mut().unwrap();
        let texture = sprite
            .get_sprite_frames()
            .unwrap()
            .get_frame_texture(&sprite.get_animation().to_string(), 0)
            .unwrap();
        let texture_2 = other_sprite
            .get_sprite_frames()
            .unwrap()
            .get_frame_texture(&sprite.get_animation().to_string(), 0)
            .unwrap();

        DisplayServer::singleton().window_set_size(Vector2i::max(
            Vector2i::new(
                texture.get_width() * scale.cast_int().x,
                texture.get_height() * scale.cast_int().y,
            ) + (Vector2i::abs((offset * scale).cast_int())),
            (texture_2.get_size() * other_sprite.get_scale()).cast_int(),
        ));
    }

    #[func]
    pub fn readjust_window_size(&mut self) {
        let offset = self.sprite.as_ref().unwrap().get_offset();
        let scale = self.sprite.as_ref().unwrap().get_scale();
        let position = self.get_position();
        let sprite = self.sprite.as_mut().unwrap();
        let texture = sprite
            .get_sprite_frames()
            .unwrap()
            .get_frame_texture(&sprite.get_animation().to_string(), 0)
            .unwrap();

        DisplayServer::singleton().window_set_size(
            Vector2i::new(
                texture.get_width() * scale.cast_int().x,
                texture.get_height() * scale.cast_int().y,
            ) + (Vector2i::abs((offset * scale).cast_int())),
        );

        if sprite.get_offset().x < 0.0 || offset.y < 0.0 {
            let offset_mul = offset * scale;
            sprite.set_position(Vector2::abs(offset * scale));
            self.move_to((position + offset_mul).cast_int());
        } else {
            sprite.set_position(Vector2::ZERO);
        }
    }

    #[func]
    pub fn get_position(&self) -> Vector2 {
        let offset = self.sprite.as_ref().unwrap().get_offset();
        self.base()
            .get_window()
            .unwrap()
            .get_position()
            .to_flt_vector()
            - if offset.x < 0.0 || offset.y < 0.0 {
                offset * self.sprite.as_ref().unwrap().get_scale()
            } else {
                Vector2::ZERO
            }
    }

    #[func]
    /// Move Flowery, and handle collision
    pub fn move_and_slide(&mut self, delta: f64) {
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

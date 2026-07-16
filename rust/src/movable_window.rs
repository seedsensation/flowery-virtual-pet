use crate::vector_cast::VectorCast;
use godot::classes::{AnimatedSprite2D, Area2D, DisplayServer};
use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MovableWindow {
    #[var(pub, set = set_offset)]
    pub offset: Vector2,

    #[var(get = get_position, set = set_position)]
    pub position: Vector2,

    #[var(pub)]
    pub ignore_collision: bool,

    #[export]
    #[var(pub)]
    /// The speed at which the character moves
    pub velocity: Vector2,

    #[export]
    #[var(pub)]
    /// How much the character accelerates by
    pub acceleration: Vector2,

    #[var(pub)]
    /// The animated sprite that represents the character
    pub sprite: Option<Gd<AnimatedSprite2D>>,

    #[var(pub)]
    /// The collision area
    pub area: Option<Gd<Area2D>>,

    /// The class that this inherits from
    pub base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MovableWindow {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            offset: Vector2::ZERO,
            position: Vector2::ZERO,
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
        Rect2::new(self.get_position(), texture.get_size() * sprite.get_scale())
    }

    #[func]
    pub fn set_offset(&mut self, offset: Vector2) {
        let old_position = self.get_position();
        godot_print!("Setting offset to {offset}");
        self.offset = offset * self.sprite.as_ref().unwrap().get_scale();
        self.set_position(old_position);
    }

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
    #[inline]
    pub fn get_position(&self) -> Vector2 {
        self.base()
            .get_window()
            .unwrap()
            .get_position()
            .to_flt_vector()
            - self.offset
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
    #[inline]
    /// Move Flowery to a specific position
    pub fn move_to(&mut self, location: Vector2i) {
        self.base()
            .get_window()
            .unwrap()
            .set_position(location + self.offset.cast_int());
    }

    #[func]
    #[inline]
    pub fn set_position(&mut self, location: Vector2) {
        self.move_to(location.cast_int())
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

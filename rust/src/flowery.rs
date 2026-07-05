use godot::classes::{AnimatedSprite2D, IAnimatedSprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
struct Flowery {
    base: Base<AnimatedSprite2D>,
}

#[godot_api]
impl IAnimatedSprite2D for Flowery {
    fn init(base: Base<AnimatedSprite2D>) -> Self {
        godot_print!("Hello, world!");

        Self { base }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (std::f64::consts::PI * delta * 5.0) as f32;
        self.base_mut().rotate(radians);
    }
}

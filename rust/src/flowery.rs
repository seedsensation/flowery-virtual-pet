use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Flowery {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Flowery {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!");

        Self { base }
    }

    fn physics_process(&mut self, _delta: f64) {
        //let radians = (std::f64::consts::PI * delta * 5.0) as f32;
        //self.base_mut().rotate(radians);
    }
}

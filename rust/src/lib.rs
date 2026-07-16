use godot::prelude::*;

pub mod movable_window;
pub mod vector_cast;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

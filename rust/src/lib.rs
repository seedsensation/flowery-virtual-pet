use godot::prelude::*;

mod movable_window;
mod vector_cast;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

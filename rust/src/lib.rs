use godot::prelude::*;

mod flowery;
mod vector_cast;
mod window_management;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::prelude::*;

mod flowery;
mod window_management;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

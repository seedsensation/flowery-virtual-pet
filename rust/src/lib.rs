use godot::prelude::*;

mod flowery;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

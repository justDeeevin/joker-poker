mod classes;

use godot::prelude::*;

struct Game;

#[gdextension]
unsafe impl ExtensionLibrary for Game {}

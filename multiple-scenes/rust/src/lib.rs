mod player;
mod entry_node;
mod main_menu_node;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

use bevy::{
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

pub struct DevToolsPlugin {
    pub toggle_keycode: KeyCode,
}

impl Default for DevToolsPlugin {
    fn default() -> Self {
        Self {
            toggle_keycode: KeyCode::Backquote,
        }
    }
}

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugUiPlugin).add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(self.toggle_keycode)),
        );
    }
}

pub fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

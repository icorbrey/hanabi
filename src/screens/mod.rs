use bevy::prelude::*;

pub struct ScreensPlugin {
    pub initial_screen: Screen,
}

impl Default for ScreensPlugin {
    fn default() -> Self {
        Self {
            initial_screen: Screen::MainMenu,
        }
    }
}

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(self.initial_screen)
            .enable_state_scoped_entities::<Screen>();
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Screen {
    MainMenu,
}

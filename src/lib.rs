use bevy::{asset::AssetMetaCheck, prelude::*};
use screens::ScreensPlugin;

#[cfg(feature = "dev")]
mod dev_tools;
pub mod screens;

#[cfg(feature = "dev")]
use crate::dev_tools::DevToolsPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Hanabi".to_string(),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        );

        app.add_plugins(ScreensPlugin::default());

        #[cfg(feature = "dev")]
        app.add_plugins(DevToolsPlugin::default());
    }
}

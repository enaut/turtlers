use bevy::prelude::Plugin;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        if cfg!(debug_assertions) {
            #[cfg(feature = "inspector")]
            app.add_plugins(WorldInspectorPlugin::default());
        }
    }
}

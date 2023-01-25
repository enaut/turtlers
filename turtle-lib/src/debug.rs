use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin);
        }
    }
}

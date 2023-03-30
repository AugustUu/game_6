use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;



mod player;
pub use crate::player::PlayerPlugin;

mod camera;
pub use crate::camera::CameraPlugin;

mod loader;
pub use crate::loader::LoaderPlugin;

pub mod components;


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugin(WorldInspectorPlugin::new())

            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())

            .add_state::<GameState>()
            .add_plugin(LoaderPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin);

        // #[cfg(debug_assertions)]
        // {
        // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

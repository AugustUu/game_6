use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;


mod player;
pub use crate::player::PlayerPlugin;

mod camera;
pub use crate::camera::CameraPlugin;

mod loader;
pub use crate::loader::LoaderPlugin;

mod map;
pub use crate::map::MapPlugin;

mod weapon;
pub use crate::weapon::WeaponPlugin;

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
            .add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::default())

            .add_state::<GameState>()
            .add_plugin(LoaderPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(MapPlugin);


        // #[cfg(debug_assertions)]
        // {
        // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .add_plugin(LogDiagnosticsPlugin::default());
        //}
    }
}

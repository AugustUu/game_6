use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_ecs_ldtk::prelude::*;

use crate::components::ColliderBundle;
use crate::{GameState, loader::MapAssets, components::NameBundle};

pub struct MapPlugin;


impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(setup_map.in_schedule(OnEnter(GameState::Playing)))
        .register_ldtk_int_cell::<ColliderBundle>(1);
    }
}

pub fn setup_map(mut commands: Commands,map_assets: Res<MapAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: map_assets.test.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..Default::default()
    }).insert(NameBundle::new("map"));
}

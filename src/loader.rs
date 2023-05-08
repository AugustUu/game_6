use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::GameState;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, MapAssets>(GameState::Loading);
    }
}
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/debug_man.png")]
    pub debug_man: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 5, rows = 1, ))]
    #[asset(path = "textures/player_sprites.png")]
    pub player_sprites: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct MapAssets{
    #[asset(path = "maps/test.ldtk")]
    pub test: Handle<LdtkAsset>,
}

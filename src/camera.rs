use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

use crate::{GameState, components::Player};


#[derive(Resource)]
pub struct GameCamera(pub Entity);

impl Default for GameCamera{
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<GameCamera>()
        .add_system(startup.in_schedule(OnEnter(GameState::Playing)))
        .add_system(movement.in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn startup(mut commands: Commands, mut camera: ResMut<GameCamera>) {

    camera.0 = commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 480.0,
                max_height: 480.0,
            },
            ..default()
        },
        ..default()
    }).id();

}

fn movement(mut query: Query<&mut Transform,With<Camera>>, windows: Query<&Window, With<PrimaryWindow>>, mut player: Query<&mut Player>){
    
    let window = windows.single();

    if let Some(position) = window.cursor_position(){
        if let Ok(mut transform) = query.get_single_mut(){
            let mut offset = position / Vec2::new(window.resolution.physical_width() as f32,window.resolution.physical_height() as f32) - Vec2::splat(0.5); // mouse offset
            offset *= 50.0;
            player.single_mut().direction = offset;
            transform.translation = Vec3::new(offset.x,offset.y,0.0);
        }
    }
    
}
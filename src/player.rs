use crate::{components::{PlayerBundle, Player, ColliderBundle, NameBundle}, loader::TextureAssets, GameState};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(startup.in_schedule(OnEnter(GameState::Playing)))
        .add_system(movement.in_set(OnUpdate(GameState::Playing)));
    }
}

fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    let camera = commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 480.0,
                max_height: 480.0,
            },
            ..default()
        },
        ..default()
    }).id();

    commands.spawn(PlayerBundle {
        sprite: SpriteBundle {
            texture: textures.debug_man_small.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        collider_bundle: ColliderBundle { 
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::cuboid(8.0, 8.0),
            locked_axes: LockedAxes::ROTATION_LOCKED
        },
        ..default()
    })
    .insert(GravityScale(0.0))
    .insert(NameBundle::new("player"))
    .add_child(camera);

    commands.spawn(ColliderBundle::default());
}

fn movement(mut query: Query<&mut Velocity,With<Player>>, keyboard_input: Res<Input<KeyCode>>){

    if let Ok(mut velocity) = query.get_single_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }
        
        //println!("{}",transform.translation);
        velocity.linvel = move_delta * 100.0;
    }
}
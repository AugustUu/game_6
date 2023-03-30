use std::time::Duration;

use crate::{components::{PlayerBundle, Player, ColliderBundle, NameBundle}, loader::TextureAssets, GameState, camera::{self, GameCamera},};
use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(startup.in_schedule(OnEnter(GameState::Playing)).after(camera::startup))
        .add_system(movement.in_set(OnUpdate(GameState::Playing)))
        .add_system(animation.in_set(OnUpdate(GameState::Playing)));
    }
}

fn startup(mut commands: Commands, textures: Res<TextureAssets>, camera: Res<GameCamera>) {
    commands.spawn(PlayerBundle {
        player: Player { 
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating), 
            ..default() 
        },
        sprite: SpriteSheetBundle {
            texture_atlas: textures.player_sprites.clone(),
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
    .add_child(camera.0);

    commands.spawn(ColliderBundle::default());
}


fn movement(mut query: Query<(&mut Player, &mut Velocity)>, keyboard_input: Res<Input<KeyCode>>){

    if let Ok((mut player,mut velocity)) = query.get_single_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        player.walking = up || down || left || right;

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

fn animation(time: Res<Time>,mut query: Query<(&mut Player,&mut TextureAtlasSprite)>){
    if let Ok((mut player,mut texture)) = query.get_single_mut() {
        texture.flip_x = player.direction.x > 0.0;

        player.animation_timer.tick(time.delta());

        if player.walking{
            player.animation_timer.set_duration(Duration::from_secs_f32(0.1));
        }else{
            player.animation_timer.set_duration(Duration::from_secs_f32(0.5));
        }

        if player.animation_timer.just_finished() {
            if player.walking{
                texture.index = match texture.index{
                    0..=1 => texture.index+1,
                    2 => 0,
                    _ => 0,
                }
            }else{
                texture.index = match texture.index{
                    3 => 0,
                    0 => 3,
                    _ => 0,
                }
            }

        }
        
        
    }
}
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
    .insert(GravityScale(1.0))
    .insert(NameBundle::new("player"))
    .add_child(camera.0);

}


fn movement(mut query: Query<(&mut Player, &mut Velocity)>, keyboard_input: Res<Input<KeyCode>>){

    if let Ok((mut player,mut velocity)) = query.get_single_mut() {
        let jump = keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Up);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        player.walking =  left || right;

        let x_axis = -(left as i8) + right as i8;
        


        if jump && velocity.linvel.y < 0.01 && velocity.linvel.y >= 0.0 {
            println!("jump");
            velocity.linvel.y += 100.0
        }else{
            if jump{
                println!("{}", velocity.linvel.y);
            }
        }
        
        //println!("{}",transform.translation);
        if player.walking{
            velocity.linvel.x = x_axis as f32 * 100.0
        }
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
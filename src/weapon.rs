use bevy::prelude::*;


pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Component, Default, Clone)]
pub struct Weapon{
    pub ammo: i32,
    pub reserve: i32,
    pub capacity: i32,
    pub dammage: i32,
    pub spread: i32,
    pub firerate: i32,
    pub reaload_speed: i32,
}


#[derive(Bundle, Default)]
pub struct WeaponBundel {
    #[bundle]
    pub sprite: SpriteBundle,

    pub weapon: Weapon,



}
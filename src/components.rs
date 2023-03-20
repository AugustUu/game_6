use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle, Default)]
pub struct NameBundle{
    name: Name
}
impl NameBundle{
    pub fn new(name:&'static str) -> Self{ 
        Self{name:Name::new(name)}
    }
}

#[derive(Component, Default, Clone)]
pub struct Player {}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub sprite: SpriteBundle,

    #[bundle]
    pub collider_bundle: ColliderBundle,
}


#[derive(Bundle,Clone,)]
pub struct ColliderBundle{
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub locked_axes: LockedAxes
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self { 
            rigidbody: RigidBody::Fixed,
            velocity: Velocity::zero(),
            collider: Collider::cuboid(16.0, 16.0),
            locked_axes: LockedAxes::ROTATION_LOCKED
        }
    }
}
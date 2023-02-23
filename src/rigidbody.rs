use std::primitive;

use bevy::prelude::{Vec2, *};

use crate::collision_primitives::{CircleCollider, CollisionPrimitive, Primitives};

#[derive(Component)]
pub struct Rigidbody {
    pub shape: Primitives,
    pub linear_velocity: Vec2,
    pub force: Vec2,
    pub angular_velocity: f32,
    pub torque: f32,
    pub restitution: Restitution,
    pub mass: f32,
    pub moment: f32,
}

impl Rigidbody {
    pub fn new_shape(mass: f32, radius: f32, primitive: Primitives) -> Self {
        let I = 0.5 * mass * radius.powf(2.);
        Self {
            shape: primitive,
            linear_velocity: Vec2::new(0.,500.),
            force: Vec2::new(0.,0.),
            angular_velocity: 0.,
            torque: 0.,
            restitution: Restitution::new(1.),
            mass: mass,
            moment: I
        }
    }

    // pub fn new(shape: dyn CollisionPrimitive, mass: f32) {
    //     Self {
    //         shape,
    //         linear_velocity: Vec2::new(0.,500.),
    //         force: Vec2::new(0.,0.),
    //         angular_velocity: 0.,
    //         torque: 0.,
    //         restitution: Restitution::new(0.1),
    //         mass,
    //         moment: 0.
    //     }
    // }

    pub fn new_shape_with_velocity(mass: f32, radius: f32, primitive: Primitives, velocity: Vec2) -> Self {
        let I = 0.5 * mass * radius.powf(2.);
        Self {
            shape: primitive,
            linear_velocity: velocity,
            force: Vec2::new(0.,0.),
            angular_velocity: 0.,
            torque: 0.,
            restitution: Restitution::new(1.),
            mass: mass,
            moment: I
        }
    }
}

#[derive(Debug, Clone)]
pub struct Restitution {
    r: f32
}

impl Restitution {
    pub fn new(r: f32) -> Self {
        if r > 1. {
            Self{r: 1.}
        }
        else if r < 0. {
            Self{r: 0.}
        }
        else {
            Self{r}
        }
    }

    pub fn resolve_restitutions(&self, other: &Restitution) -> f32 {
        self.r.min(other.r)
    }
}

impl Default for Restitution {
    fn default() -> Self {Self {r: 1.}}
}
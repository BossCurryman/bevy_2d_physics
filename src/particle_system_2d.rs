use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
#[derive(Bundle, Default)]
pub struct Particle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub velocity: Velocity,
    pub mass: Mass,
}

#[derive(Component, Default)]
pub struct Mass {
    pub m: f32,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub v: Vec2,
}
impl Velocity {
    pub fn new(mag: f32, direction: Vec2) -> Self {
        Velocity { v: direction.normalize_or_zero() * mag }
    }
}

pub fn step_velocity(
    mut particles: Query<(&mut Velocity, &mut Transform)>,
    timer: Res<Time>,
) {
    for (vel, mut trans) in particles.iter_mut() {
        let vel_3d = Vec3::from((vel.v, 0.0));
        trans.translation += vel_3d * timer.delta_seconds();
    }
}

pub fn compute_collisions(
    mut particles: Query<(&mut Velocity, &Transform, &Mass)>,
) {
    let mut combinations = particles.iter_combinations_mut();
    while let Some([(mut vel_1, trans_1, m_1),
        (mut vel_2, trans_2, m_2)]) = combinations.fetch_next() {
        if trans_1.translation.distance(trans_2.translation) < 40. {
            // Find normal of particles
            let normal_1 = (vel_1.v - vel_2.v) /2.;
            let normal_2 = (vel_2.v - vel_1.v) /2.;
            // Find tangential of particles
            let tangential_1 = (vel_1.v + vel_2.v) /2.;
            let tangential_2 = (vel_2.v + vel_1.v) /2.;
            println!("Close enough, p1: {}, p2: {}", vel_1.v.length() * m_1.m, vel_2.v.length() * m_2.m);
            println!("normal 1{}, normal 2{}, tangential 1{}, tangential 2 {}", normal_1, normal_2, tangential_1, tangential_2);

            const rest: f32 = 1.;
            let mass_ratio = m_1.m/m_2.m;
            let final_normal_1 = (mass_ratio*normal_1 + normal_2 - normal_1 * rest + normal_2 * rest)/(1. + mass_ratio);
            let final_normal_2 = (m_1.m * normal_1 + m_2.m * normal_2 - m_1.m * final_normal_1)/m_2.m;

            let final_velocity_1 = tangential_1 + final_normal_1;
            let final_velocity_2 = tangential_2 + final_normal_2;

            vel_1.v = final_velocity_1;
            vel_2.v = final_velocity_2;
            println!("final p1 {}, p2 {}", vel_1.v.length() * m_1.m, vel_2.v.length() * m_2.m);
            
        }
    }
}
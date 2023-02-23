mod collision_primitives;
mod rigidbody;
mod particle_system_2d;

use bevy::{prelude::*, sprite::{Mesh2dHandle, Material2d}, ecs::query::WorldQuery};
use collision_primitives::{CircleCollider, CollisionPrimitive, Primitives, CollisionData, AABB};
use rand::prelude::*;
use rigidbody::Rigidbody;

#[derive(Bundle)]
struct RigidbodyBundle<M: Material2d> {
    transform: Transform,
    global_transform: GlobalTransform,
    mesh: Mesh2dHandle,
    material: Handle<M>,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    rigidbody: Rigidbody
}

#[derive(Resource)]
struct PhysicsTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PhysicsTimer(Timer::from_seconds(0.016667, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(step_physics)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation((0.,0.,0.).into()),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Circle::new(40.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PINK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_shape_with_velocity(65., 40., Primitives::Circle(CircleCollider::new(40.)), Vec2::new(150.,0.))
    });
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation((120.,30.,0.).into()),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_shape_with_velocity(20., 20., Primitives::Circle(CircleCollider::new(20.)), Vec2::new(0.,0.))
    });

    let mut rng = thread_rng();
    // for i in 0..20 {
    //     let mag: f32 = rng.gen_range(10.0..100.0);
    //     let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
    //     let pos = Vec3::new(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0), 0.);
    //     commands.spawn(RigidbodyBundle {
    //         transform: Transform::from_translation(pos),
    //         global_transform: GlobalTransform::default(),
    //         mesh: meshes.add(shape::Circle::new(20.).into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::BLACK)),
    //         visibility: Visibility::VISIBLE,
    //         computed_visibility: ComputedVisibility::default(),
    //         rigidbody: Rigidbody::new_with_velocity(1., 20., mag * dir )
    //     });
    // }
    let size_1 = Vec2::new(20.,20.);
    let prim_1 = AABB::new(size_1.x, size_1.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(-100.,-150.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(size_1).into()).into(),
        material: materials.add(ColorMaterial::from(Color::ORANGE)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_shape_with_velocity(30., 1., Primitives::AABB(prim_1) , Vec2::new(100., 0.))
    });

    let size_2 = Vec2::new(20.,20.);
    let prim_2 = AABB::new(size_1.x, size_1.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(0.,-150.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(size_2).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PINK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_shape_with_velocity(50., 1., Primitives::AABB(prim_2) , Vec2::new(0., 0.))
    });
}

fn step_physics(
    mut rigidbodies: Query<(&mut Rigidbody, &mut Transform)>,
    time: Res<Time>,
    mut phys_timer: ResMut<PhysicsTimer>) {
    if phys_timer.0.tick(time.delta()).just_finished() {

        // println!("Phys step {}", phys_timer.0.elapsed_secs());
        rigidbodies.for_each_mut(|(mut r, mut t)| {
            // Apply forces



            // Update Velocities and positions
            let linear_acceleration = r.force / r.mass;
            r.linear_velocity += linear_acceleration;
            t.translation += Vec3::from((r.linear_velocity, 0.)) * 0.016667;
        });
        let mut combinations = rigidbodies.iter_combinations_mut();

        // Detect collisions
        while let Some([
            (mut rigidbody_1, transform_1),
            (mut rigidbody_2, transform_2)
        ]) = combinations.fetch_next() {
            match &rigidbody_1.shape {
                Primitives::Circle(c1) => {
                    if let Some(collision_data) = check_circle_collision(c1, &transform_1, &rigidbody_2, &transform_2) {
                        println!("Collision! normal: {} , depth: {} ", collision_data.unit_normal, collision_data.penetration_depth);
                        resolve_collision(collision_data, &mut rigidbody_1, &mut rigidbody_2)
                    }
                }
                Primitives::AABB(c1) => {
                    if let Some(collision_data) = check_AABB_collision(c1, &transform_1, &rigidbody_2, &transform_2) {
                        println!("AABB Collision! normal: {}, depth: {}", collision_data.unit_normal, collision_data.penetration_depth);
                        resolve_collision(collision_data, &mut rigidbody_1, &mut rigidbody_2)
                    }
                }
            }
        }
    }
}

// Yucky ass function, all over the place. (Impure, weird conditional return, )
fn resolve_collision(collision_data: CollisionData,rigidbody_1: &mut Rigidbody, rigidbody_2: &mut Rigidbody) -> () {
    // Resolve collision
    let relative_velocity =  rigidbody_1.linear_velocity - rigidbody_2.linear_velocity;
    let veloctiy_along_normal = relative_velocity.dot(collision_data.unit_normal);

    // NOTE: Do not resolve velocities if the objects are seperating. but if they are seperating, how did they collide?
    if veloctiy_along_normal > 0. {
        return 
    }
    // Calculate restitution
    let restitution = rigidbody_1.restitution.resolve_restitutions(&rigidbody_2.restitution);

    let mass_1 = rigidbody_1.mass;
    let mass_2 = rigidbody_2.mass;
    let impulse_scalar = (-(1. + restitution) * veloctiy_along_normal) / (1./ mass_1 + 1./ mass_2);
    
    // Apply impulse
    let impulse = impulse_scalar * collision_data.unit_normal;
    rigidbody_1.linear_velocity += impulse / mass_1;
    rigidbody_2.linear_velocity -= impulse / mass_2;
}

fn check_circle_collision(circle: &CircleCollider, circle_trans: &Transform, other: &Rigidbody, other_trans: &Transform) -> Option<CollisionData> {
    match &other.shape {
        Primitives::Circle(c2) => {
            circle.is_colliding_with_circle(circle_trans, c2, other_trans)
        }
        Primitives::AABB(c2) => {
            None
        }
    }
}

fn check_AABB_collision(AABB: &AABB, AABB_trans: &Transform, other: &Rigidbody, other_trans: &Transform ) -> Option<CollisionData> {
    match &other.shape {
        Primitives::AABB(c2) => {
            AABB.is_colliding_with_AABB(AABB_trans, c2, other_trans)
        }
        Primitives::Circle(c2) => {
            None
        }
    }
}



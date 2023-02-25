mod collision_primitives;
mod rigidbody;
mod particle_system_2d;

use bevy::{prelude::*, sprite::{Mesh2dHandle, Material2d}, ecs::query::WorldQuery};
use collision_primitives::{CircleCollider, CollisionPrimitive, Primitives, CollisionData, AABB};
use rand::prelude::*;
use rigidbody::{Rigidbody, Mass};

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
        transform: Transform::from_translation((-150.,28.,0.).into()),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PINK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_shape_with_velocity(65., 20., Primitives::Circle(CircleCollider::new(20.)), Vec2::new(200.,0.))
    });
    // commands.spawn(RigidbodyBundle {
    //     transform: Transform::from_translation((120.,30.,0.).into()),
    //     global_transform: GlobalTransform::default(),
    //     mesh: meshes.add(shape::Circle::new(20.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::BLACK)),
    //     visibility: Visibility::VISIBLE,
    //     computed_visibility: ComputedVisibility::default(),
    //     rigidbody: Rigidbody::new_shape_with_velocity(20., 20., Primitives::Circle(CircleCollider::new(20.)), Vec2::new(0.,0.))
    // });

    let mut rng = thread_rng();
    for i in 0..20 {
        let mag: f32 = rng.gen_range(100.0..1000.0);
        let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let pos = Vec3::new(rng.gen_range(-100.0..100.0), rng.gen_range(-300.0..300.0), 0.);
        commands.spawn(RigidbodyBundle {
            transform: Transform::from_translation(pos),
            global_transform: GlobalTransform::default(),
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            visibility: Visibility::VISIBLE,
            computed_visibility: ComputedVisibility::default(),
            rigidbody: Rigidbody::new_shape_with_velocity(30., 20., Primitives::Circle(CircleCollider::new(20.)),mag * dir )
        });
    }
    let size_1 = Vec2::new(80.,90.);
    let prim_1 = AABB::new(size_1.x, size_1.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(0.,0.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(size_1).into()).into(),
        material: materials.add(ColorMaterial::from(Color::ORANGE)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_shape_with_velocity(30., 1., Primitives::AABB(prim_1) , Vec2::new(50., 0.))
    });

    let size_2 = Vec2::new(50.,50.);
    let prim_2 = AABB::new(size_2.x, size_2.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(25.,0.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(size_2).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PINK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_static_shape(Primitives::AABB(prim_2))
    });

    let top = Vec2::new(500.,10.);
    let prim_top = AABB::new(top.x, top.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(0.,405.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(top).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_static_shape(Primitives::AABB(prim_top))
    });
    let bottom = Vec2::new(500.,10.);
    let prim_bottom = AABB::new(bottom.x, bottom.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(0.,-405.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(bottom).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_static_shape(Primitives::AABB(prim_bottom))
    });
    let left = Vec2::new(10.,800.);
    let prim_left = AABB::new(left.x, left.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(-255.,0.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(left).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_static_shape(Primitives::AABB(prim_left))
    });
    let right = Vec2::new(10.,800.);
    let prim_right = AABB::new(right.x, right.y);
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation(Vec3::new(255.,0.,0.)),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Quad::new(right).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_static_shape(Primitives::AABB(prim_right))
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
            if let Mass::Some(m) = r.mass {
                let linear_acceleration = r.force / m;
                r.linear_velocity += linear_acceleration;
                t.translation += Vec3::from((r.linear_velocity, 0.)) * 0.016667;
            }
            // If object is static, no kinematics need to take place
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
                        // println!("Collision! normal: {} , depth: {} ", collision_data.unit_normal, collision_data.penetration_depth);
                        resolve_collision(collision_data, &mut rigidbody_1, &mut rigidbody_2)
                    }
                }
                Primitives::AABB(c1) => {
                    if let Some(collision_data) = check_AABB_collision(c1, &transform_1, &rigidbody_2, &transform_2) {
                        // println!("AABB Collision! normal: {}, depth: {}", collision_data.unit_normal, collision_data.penetration_depth);
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

    let invert_mass_1;
    if let Mass::Some(m) = rigidbody_1.mass {
        invert_mass_1 = 1./m
    }
    else {
        invert_mass_1 = 0.;
    }
    let invert_mass_2;
    if let Mass::Some(m) = rigidbody_2.mass {
        invert_mass_2 = 1./m
    }
    else {
        invert_mass_2 = 0.;
    }
    let impulse_scalar = (-(1. + restitution) * veloctiy_along_normal) / (invert_mass_1 + invert_mass_2);
    
    // Apply impulse
    let impulse = impulse_scalar * collision_data.unit_normal;
    rigidbody_1.linear_velocity += impulse * invert_mass_1;
    rigidbody_2.linear_velocity -= impulse * invert_mass_2;
}

fn check_circle_collision(circle: &CircleCollider, circle_trans: &Transform, other: &Rigidbody, other_trans: &Transform) -> Option<CollisionData> {
    match &other.shape {
        Primitives::Circle(c2) => {
            circle.is_colliding_with_circle(circle_trans, c2, other_trans)
        }
        Primitives::AABB(c2) => {
            circle.is_colliding_with_AABB(circle_trans, c2, other_trans)
        }
    }
}

fn check_AABB_collision(AABB: &AABB, AABB_trans: &Transform, other: &Rigidbody, other_trans: &Transform ) -> Option<CollisionData> {
    match &other.shape {
        Primitives::AABB(c2) => {
            AABB.is_colliding_with_AABB(AABB_trans, c2, other_trans)
        }
        Primitives::Circle(c2) => {
            AABB.is_colliding_with_circle(AABB_trans, c2, other_trans)
        }
    }
}



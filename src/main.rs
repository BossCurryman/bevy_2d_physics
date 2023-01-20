mod collision_primitives;
mod rigidbody;
mod particle_system_2d;

use bevy::{prelude::*, sprite::{Mesh2dHandle, Material2d}, transform};
use collision_primitives::{CircleCollider, CollisionPrimitive};
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
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PINK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_with_velocity(1., 20., Vec2::new(150.,0.))
    });
    commands.spawn(RigidbodyBundle {
        transform: Transform::from_translation((120.,30.,0.).into()),
        global_transform: GlobalTransform::default(),
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        visibility: Visibility::VISIBLE,
        computed_visibility: ComputedVisibility::default(),
        rigidbody: Rigidbody::new_with_velocity(1., 20., Vec2::new(-50., 0.) )
    });

    let mut rng = thread_rng();
    for i in 0..20 {
        let mag: f32 = rng.gen_range(10.0..100.0);
        let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let pos = Vec3::new(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0), 0.);
        commands.spawn(RigidbodyBundle {
            transform: Transform::from_translation(pos),
            global_transform: GlobalTransform::default(),
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            visibility: Visibility::VISIBLE,
            computed_visibility: ComputedVisibility::default(),
            rigidbody: Rigidbody::new_with_velocity(1., 20., mag * dir )
        });
    }
}


fn step_physics(
    mut rigidbodies: Query<(&mut Rigidbody<dyn CollisionPrimitive>, &mut Transform)>,
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
            (mut rigidbody1, transform1),
            (mut rigidbody2, transform2)
        ]) = combinations.fetch_next() {
            if let Some(collision_data) = 
                rigidbody1.shape.is_colliding_with(&transform1,&rigidbody2.shape, &transform2) {
                println!("Collision! normal: {} , depth: {} ", collision_data.unit_normal, collision_data.penetration_depth);
                // Resolve collision
                let relative_velocity =  rigidbody1.linear_velocity - rigidbody2.linear_velocity;
                let veloctiy_along_normal = relative_velocity.dot(collision_data.unit_normal);

                // NOTE: Do not resolve velocities if the objects are seperating. but if they are seperating, how did they collide?
                if veloctiy_along_normal > 0. {
                    continue
                }
                // Calculate restitution
                let restitution = rigidbody1.restitution.resolve_restitutions(&rigidbody2.restitution);

                let impulse_scalar = (-(1. + restitution) * veloctiy_along_normal) / (1./ rigidbody1.mass + 1./ rigidbody2.mass);
                
                // Apply impulse
                let impulse = impulse_scalar * collision_data.unit_normal;
                rigidbody1.linear_velocity += impulse;
                rigidbody2.linear_velocity -= impulse;
            }
        }
    }
}



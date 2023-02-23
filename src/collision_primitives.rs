use bevy::prelude::*;

pub struct CollisionData {
    pub unit_normal: Vec2,
    pub penetration_depth: f32,
}

pub enum Primitives {
    Circle(CircleCollider),
    AABB(AABB)
}

pub trait CollisionPrimitive {
    fn collide(
        &self,
        transform_self: &Transform,
        other: CircleCollider,
        transform_other: &Transform,
    )  -> Option<CollisionData>;
}

pub struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    pub fn new(length: f32, height: f32) -> Self {
        Self {
            min: Vec2::new(-length / 2., -height / 2.),
            max: Vec2::new(length / 2., height / 2.),
        }
    }

    pub fn get_extents_as_global_vectors(&self, self_transform: &Transform) -> (Vec2, Vec2) {
        (
            self_transform.translation.truncate() + self.min,
            self_transform.translation.truncate() + self.max,
        )
    }

    pub fn is_colliding_with_AABB(&self, self_transform: &Transform, other: &AABB, other_transform: &Transform) -> Option<CollisionData> {
        let (min_1, max_1) = self.get_extents_as_global_vectors(self_transform);
        let (min_2, max_2) = other.get_extents_as_global_vectors(other_transform);
        if max_1.x < min_2.x || min_1.x > max_2.x {
            return None;
        }
        if max_1.x < min_2.x || min_1.x > max_2.x {
            return None;
        }


        // Collision confirmed, find impulse normal and pen depth

        // Steps adapted from https://gamedevelopment.tutsplus.com/tutorials/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331

        // Vec from self to other
        let n = (self_transform.translation - other_transform.translation).truncate();

        let self_extent = (max_1.x - min_1.x) / 2.;
        let other_extent = (max_2.x - min_2.x) / 2.;
        let overlap_x = self_extent + other_extent - n.x.abs();

        if overlap_x > 0. {
            let self_extent = (max_1.y - min_1.y) / 2.;
            let other_extent = (max_2.y - min_2.y) / 2.;

            let overlap_y = self_extent + other_extent - n.y.abs();

            if overlap_y > 0. {
                println!("Overlap x: {} overlap y: {}", overlap_x, overlap_y);
                let normal;
                let penetration;
                if overlap_x < overlap_y {
                    if n.x < 0. {
                        normal = Vec2::new(-1., 0.);
                    } else {
                        normal = Vec2::new(1., 0.);
                    }
                    penetration = overlap_x;
                }
                else {
                    if n.y < 0. {
                        normal = Vec2::new(0., -1.);
                    } else {
                        normal = Vec2::new(0., 1.);
                    }
                    penetration = overlap_y;
                }
                return Some(CollisionData {
                    unit_normal: normal,
                    penetration_depth: penetration,
                })
            }
        }
        None


    }
}

pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_colliding_with_circle(
        &self,
        self_transform: &Transform,
        other: &CircleCollider,
        other_transform: &Transform,
    ) -> Option<CollisionData> {
        let radius_squared = (self.radius + other.radius).powi(2);
        let normal = (self_transform.translation - other_transform.translation).truncate();
        if radius_squared > normal.length_squared() {
            //Collision!
            let distance = normal.length();
            let penetration_depth = radius_squared.sqrt() - distance;
            let unit_normal = normal / distance;
            Some(CollisionData {
                unit_normal,
                penetration_depth,
            })
        } else {
            None
        }
    }

    pub fn is_colliding_with_AABB(
        &self,
        self_transform: &Transform,
        other: &AABB,
        other_transform: &Transform,
    ) -> Option<CollisionData> {
        None
    }
}

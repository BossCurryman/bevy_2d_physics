use std::ops::{Rem, Neg};

use bevy::prelude::{shape::Circle, *};

pub struct CollisionData {
    pub unit_normal: Vec2,
    pub penetration_depth: f32,
}

pub enum Primitives {
    Circle(CircleCollider),
    AABB(AABB),
}

pub trait CollisionPrimitive {
    fn collide(
        &self,
        transform_self: &Transform,
        other: CircleCollider,
        transform_other: &Transform,
    ) -> Option<CollisionData>;
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

    pub fn get_extents_as_array_of_verteces(&self, self_transform: &Transform) -> [Vec2; 4] {
        let (world_min, world_max) = self.get_extents_as_global_vectors(self_transform);
        [
            world_max,
            Vec2::new(world_max.x, world_min.y),
            world_min,
            Vec2::new(world_min.x, world_max.y),
        ]
    }

    pub fn get_extents_as_global_vectors(&self, self_transform: &Transform) -> (Vec2, Vec2) {
        (
            self_transform.translation.truncate() + self.min,
            self_transform.translation.truncate() + self.max,
        )
    }

    pub fn is_colliding_with_AABB(
        &self,
        self_transform: &Transform,
        other: &AABB,
        other_transform: &Transform,
    ) -> Option<CollisionData> {
        let (min_1, max_1) = self.get_extents_as_global_vectors(self_transform);
        let (min_2, max_2) = other.get_extents_as_global_vectors(other_transform);
        if max_1.x < min_2.x || min_1.x > max_2.x {
            return None;
        }
        if max_1.y < min_2.y || min_1.y > max_2.y {
            return None;
        }
        // Collision confirmed, find impulse normal and pen depth

        // Steps adapted from https://gamedevelopment.tutsplus.com/tutorials/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331
        // TODO: clean up this algo. much smarter ways of doing most of these things

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
                // println!("Overlap x: {} overlap y: {}", overlap_x, overlap_y);
                let normal;
                let penetration;
                if overlap_x < overlap_y {
                    if n.x < 0. {
                        normal = Vec2::new(-1., 0.);
                    } else {
                        normal = Vec2::new(1., 0.);
                    }
                    penetration = overlap_x;
                } else {
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
                });
            }
        }
        None
    }

    pub fn is_colliding_with_circle(
        &self,
        self_transform: &Transform,
        other: &CircleCollider,
        other_transform: &Transform,
    ) -> Option<CollisionData> {
        // Check collision
        // check which voronoi region the circle is in
        // the closest vecctor + the two vectors that make edges with it
        let verts = self.get_extents_as_array_of_verteces(self_transform);
        let mut closest = Vec2::new(f32::MAX, f32::MAX);
        let mut edge_infront = self_transform.translation.truncate();
        let mut edge_behind = self_transform.translation.truncate();

        let center = other_transform.translation.truncate();
        for (i, vert) in verts.into_iter().enumerate() {
            if vert.distance(center) < closest.distance(center) {
                closest = vert;
                // println!("{}", i-1);
                edge_behind = *verts.get((i + 3).rem_euclid(4))?;
                // println!("{edge_behind}");

                edge_infront = *verts.get((i + 1)%4)?;
            }
        }

        let axis_behind = edge_behind - closest;
        let axis_infront = edge_infront - closest;
        let reframed_center = center - closest;

        // SAT requires at least world alined axis overlap
        let (min_1, max_1) = self.get_extents_as_global_vectors(self_transform);
        let (min_2, max_2) = other.get_extents_on_world_axes(other_transform);
        is_overlaping_both_world_axes(min_1, max_1, min_2, max_2)?;

        let normal;
        let pen_depth;

        if reframed_center.dot(axis_behind) > 0. {
            // Edge hit on behind edge
            // Voronoi region is edge
            pen_depth = is_overlaping_both_world_axes(min_1, max_1, min_2, max_2)?.min_element();
            // println!("Edge collision");
            // Collision!
            // Find the normal of the edge (cross product)
            normal = reframed_center.reject_from(axis_behind).normalize();
            // let normal = center.reject_from(c1 - c2).normalize();
        }
        else if reframed_center.dot(axis_infront) > 0. {
            // Edge hit on behind edge
            // Voronoi region is edge
            pen_depth = is_overlaping_both_world_axes(min_1, max_1, min_2, max_2)?.min_element();
            // println!("Edge collision");
            // Collision!
            // Find the normal of the edge (vector rejection)
            normal = reframed_center.reject_from(axis_infront).normalize();
        } 
        else {
            // Voronoi region is corner
            // SAT requires world aligned axis overlap + corner -> center axis overlap
            let projection_axis = (center - closest).normalize();

            // project AABB onto axis
            let projections_aabb = verts.map(|v| v.dot(projection_axis));
            let max_aabb = projections_aabb
                .iter()
                .copied()
                .max_by(|a, b| a.total_cmp(b))?;
            let min_aabb = projections_aabb
                .iter()
                .copied()
                .min_by(|a, b| a.total_cmp(b))?;

            // project circle onto axis
            let min_circle = center.dot(projection_axis) - other.radius;
            let max_circle = center.dot(projection_axis) + other.radius;

            normal = projection_axis;
            pen_depth = sat_is_overlaping(min_aabb, max_aabb, min_circle, max_circle)?;
            // Collision!
            // println!("Corner collision");
        }
        Some(CollisionData {
            unit_normal: normal,
            penetration_depth: pen_depth,
        })
        
    }
}

fn sat_is_overlaping(min_1: f32, max_1: f32, min_2: f32, max_2: f32) -> Option<f32> {
    if max_1 > min_2 && min_1 < max_2 {
        Some((max_2 - min_1).min(max_1 - min_2))
    } else {
        None
    }
}

/// Checks both world alligned axes for overlaps on the given mins and maxes.
/// Returns None if thesere is one axis without overlap or Some Vec2 with penetration depths of each axis
fn is_overlaping_both_world_axes(
    min_1: Vec2,
    max_1: Vec2,
    min_2: Vec2,
    max_2: Vec2,
) -> Option<Vec2> {
    if (max_1.x > min_2.x && min_1.x < max_2.x) && (max_1.y > min_2.y && min_1.y < max_2.y) {
        Some((max_2 - min_1).min(max_1 - min_2))
    } else {
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

    // Inline candidate
    fn get_extents_on_world_axes(&self, self_transform: &Transform) -> (Vec2, Vec2) {
        (
            self_transform.translation.truncate() - self.radius,
            self_transform.translation.truncate() + self.radius,
        )
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
        other.is_colliding_with_circle(other_transform, self, self_transform)
    }
}

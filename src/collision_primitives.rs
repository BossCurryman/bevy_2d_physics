use bevy::prelude::*;

pub struct CollisionData {
    pub unit_normal: Vec2,
    pub penetration_depth: f32,
}

pub trait CollisionPrimitive<T: CollisionPrimitive<T>> {

    fn collide(
        &self,
        transform_self: &Transform,
        other: &T,
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

    pub fn get_extents_as_global_vectors(&self, self_transform: &Transform) -> (Vec2, Vec2) {
        (
            self_transform.translation.truncate() + self.min,
            self_transform.translation.truncate() + self.max,
        )
    }

    pub fn is_colliding_with(&self, self_transform: &Transform, other: &AABB, other_transform: &Transform) -> Option<CollisionData> {
        let (min_1, max_1) = self.get_extents_as_global_vectors(self_transform);
        let (min_2, max_2) = other.get_extents_as_global_vectors(other_transform);
        if max_1.x < min_2.x || min_1.x > max_2.x {
            return None;
        }
        if max_1.x < min_2.x || min_1.x > max_2.x {
            return None;
        }
        Some(CollisionData {
            unit_normal: default(),
            penetration_depth: 0.,
        })
    }
}
impl CollisionPrimitive<AABB> for AABB {
    
    fn collide(
            &self,
            transform_self: &Transform,
            other: &AABB,
            transform_other: &Transform,
        ) -> Option<CollisionData> {
        let normal = (transform_self.translation - transform_other.translation).truncate();
        
        let extent_self = (self.max - self.min) / 2.;
        let extent_other = (other.max - other.min) / 2.;

        let overlap = extent_self + extent_other - normal;

        if overlap.x > 0. || overlap.y > 0. {
            if overlap.x > overlap.y {
                let unit_normal = 
                if normal.x  < 0. {
                    Vec2::new(-1.,0.)
                }
                else {
                    Vec2::new(1.,0.)
                };
                return Some(CollisionData{unit_normal, penetration_depth: overlap.x})
            }
            else{
                let unit_normal = 
                if normal.x  < 0. {
                    Vec2::new(0.,-1.)
                }
                else {
                    Vec2::new(0.,1.)
                };
                return Some(CollisionData{unit_normal, penetration_depth: overlap.x})
            }
        }
        else{
            None
        }
        
    }
}

pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_colliding_with(
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
}

impl CollisionPrimitive<CircleCollider> for CircleCollider {
    
    fn collide(
            &self,
            transform_self: &Transform,
            c: &CircleCollider,
            transform_c: &Transform,
        ) -> Option<CollisionData> {
            
        let radius_squared = (self.radius + c.radius).powi(2);
        let normal = (transform_self.translation - transform_c.translation).truncate();
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
}

impl CollisionPrimitive<AABB> for CircleCollider {
    
    fn collide(
            &self,
            transform_self: &Transform,
            c: &AABB,
            transform_c: &Transform,
        ) -> Option<CollisionData> {
        None
    }
}

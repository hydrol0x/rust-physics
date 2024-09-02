use nalgebra::Vector2;
pub enum Constraint {
    Distance(DistanceConstraint),
    Spring(SpringConstraint),
    FixedPoint(FixedPointConstraint),
}

#[derive(Default)]
pub struct ConstraintUpdate {
    pub index: usize,
    pub velocity_update: Vector2<f32>,
    pub position_update: Vector2<f32>,
    pub force_update: Vector2<f32>,
}

#[derive(Default)]
pub struct DistanceConstraint {
    pub index_0: usize,
    pub index_1: usize,
    pub distance: f32,
}
impl DistanceConstraint {
    pub fn new(index_0: usize, index_1: usize, distance: f32) -> Self {
        Self {
            index_0: index_0,
            index_1: index_1,
            distance: distance,
        }
    }
}

#[derive(Default, Debug)]
pub struct SpringConstraint {
    pub index_0: usize,
    pub index_1: usize,
    pub distance: f32,
    pub k: f32,
    pub dampen: f32,
}

pub struct FixedPointConstraint {
    pub index: usize,
    pub position: Vector2<f32>,
}

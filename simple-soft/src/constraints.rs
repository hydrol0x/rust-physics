pub enum Constraint {
    Distance(DistanceConstraint),
    Spring(SpringConstraint),
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
}

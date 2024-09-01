enum Constraint {
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

pub struct TriangleConstraint {
    pub edge_1: DistanceConstraint,
    pub edge_2: DistanceConstraint,
    pub edge_3: DistanceConstraint,
}

impl TriangleConstraint {
    pub fn new(
        index_0: usize,
        index_1: usize,
        index_2: usize,
        side_1_length: f32,
        side_2_length: f32,
        side_3_length: f32,
    ) -> Self {
        let edge_1 = DistanceConstraint::new(index_0, index_1, side_1_length);
        let edge_2 = DistanceConstraint::new(index_1, index_2, side_2_length);
        let edge_3 = DistanceConstraint::new(index_2, index_0, side_3_length);
        Self {
            edge_1,
            edge_2,
            edge_3,
        }
    }
}

pub struct SpringConstraint {
    pub index_0: usize,
    pub index_1: usize,
    pub distance: f32,
    pub k: f32,
}

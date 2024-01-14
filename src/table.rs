use crate::geometry::Point;

pub struct BeamTable {
    pub events: Vec<Point>,
    pub actives: Vec<Vec<i32>>,
}


impl BeamTable {
    pub(crate) fn new() -> BeamTable {
        BeamTable {
            events: Vec::new(),
            actives: Vec::new(),
        }
    }
}

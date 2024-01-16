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

    pub fn actives_at(&self, x: f64, y: f64) -> &Vec<i32> {
        let idx = self.events.binary_search(&Point::new(x, y));
        match idx {
            Ok(value) => {
                return &self.actives[value];
            }
            Err(value) => {
                if value == 0 {
                    return &self.actives.last().expect("at least 1 active must exist.");
                }
                let value = value.checked_sub(1).unwrap();
                return &self.actives[value];
            }
        }
    }
}

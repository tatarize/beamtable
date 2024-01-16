use vsvg::exports::egui::Shape::Vec;
use crate::geometry::{Geomstr, Point};

pub struct SpaceMask {
    pub inside: Vec<Vec<bool>>,
}

impl SpaceMask {
    pub fn new(mask: Vec<Vec<bool>>) -> SpaceMask {
        SpaceMask {
            inside: mask,
        }
    }
}

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

    pub fn evenodd_fill(&self, geometry: Geomstr, settings: f64) -> SpaceMask {
        let spacemask = Vec::new();
        for active in &self.actives {
            let active_mask = Vec::new();
            let mut inside = false;
            active_mask.push(inside);
            for a in active {
                let line = &geometry.segments[a];
                if line.2.1 == settings {
                    inside = !inside;
                }
                active_mask.push(inside);
            }
            spacemask.push(active_mask);
        }
        SpaceMask::new(spacemask)
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

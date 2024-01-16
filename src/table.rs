use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, Not};
use crate::geometry::{Geomstr, Point};

#[derive(Debug, Clone)]
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

impl BitAnd for SpaceMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut n = Vec::new();
        for j in 0..rhs.inside.len() {
            let mut m = Vec::new();
            for k in 0..rhs.inside[j].len() {
                m.push(self.inside[j][k] & rhs.inside[j][k]);
            }
            n.push(m);
        }
        Self::new(n)
    }
}


impl BitOr for SpaceMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut n = Vec::new();
        for j in 0..rhs.inside.len() {
            let mut m = Vec::new();
            for k in 0..rhs.inside[j].len() {
                m.push(self.inside[j][k] | rhs.inside[j][k]);
            }
            n.push(m);
        }
        Self::new(n)
    }
}

impl Not for SpaceMask {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut n = Vec::new();
        for j in 0..self.inside.len() {
            let mut m = Vec::new();
            for k in 0..self.inside[j].len() {
                m.push(!self.inside[j][k]);
            }
            n.push(m);
        }
        Self::new(n)
    }
}

#[derive(Debug, Clone)]
pub struct BeamTable {
    pub geometry: Geomstr,
    pub events: Vec<Point>,
    pub actives: Vec<Vec<i32>>,
}

impl BeamTable {
    pub(crate) fn new(geometry: Geomstr) -> BeamTable {
        BeamTable {
            geometry,
            events: Vec::new(),
            actives: Vec::new(),
        }
    }

    pub fn evenodd_fill(&self, settings: f64) -> SpaceMask {
        let mut spacemask = Vec::new();
        for active in &self.actives {
            let mut active_mask = Vec::new();
            let mut inside = false;
            active_mask.push(inside);
            for a in active {
                let line = &self.geometry.segments[*a as usize];
                if line.2.1 == settings {
                    inside = !inside;
                }
                active_mask.push(inside);
            }
            spacemask.push(active_mask);
        }
        SpaceMask::new(spacemask)
    }

    pub fn even_odd_ignoring_origin(&self) -> SpaceMask {
        let mut spacemask = Vec::new();
        for active in &self.actives {
            let mut active_mask = Vec::new();
            let mut inside = false;
            active_mask.push(inside);
            for _a in active {
                inside = !inside;
                active_mask.push(inside);
            }
            spacemask.push(active_mask);
        }
        SpaceMask::new(spacemask)
    }

    pub fn union_all(&self) -> SpaceMask {
        let mut spacemask = Vec::new();
        for active in &self.actives {
            let mut set: HashMap<i32, bool> = HashMap::new();
            let mut active_mask = Vec::new();
            active_mask.push(set.len() != 0);
            for a in active {
                let line = &self.geometry.segments[*a as usize];
                if set.contains_key(&(line.2.1 as i32)) {
                    set.remove(&(line.2.1 as i32));
                    // println!("Removed {:?}", set);
                }
                else {
                    set.insert(line.2.1 as i32, true);
                    // println!("Added {:?}", set);
                }
                active_mask.push(set.len() != 0);
            }
            spacemask.push(active_mask);
        }
        SpaceMask::new(spacemask)
    }

    pub fn create(&self, mask: SpaceMask) -> Geomstr {
        let mut g = Geomstr::new();
        let inside = &mask.inside;
        for j in 0..inside.len() -2 {
            //mask exists at inside-1, but the final entry is actually pointless
            let prev_event = &self.events[j];
            let curr_event = &self.events[j+1];

            let beam_active = &self.actives[j];
            for k in 0..inside[j].len() -1 {
                let active = beam_active[k];
                let p = inside[j][k];
                let c;
                if k != inside[j].len() - 1 {
                    c = inside[j][k + 1];
                }
                else {
                    c = false;
                }
                if (p && !c) || (!p && c) {
                    //is a boundary.
                    let start = self.geometry.y_intercept(active as usize, prev_event.x, prev_event.y);
                    let end = self.geometry.y_intercept(active as usize, curr_event.x, curr_event.y);
                    let line = &self.geometry.segments[active as usize];
                    g.line((start.x, start.y), (end.x, end.y), line.2.1);
                }
            }
        }
        g
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

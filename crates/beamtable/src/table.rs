use crate::events::Event;
use crate::geometry::{Geomstr, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{BitAnd, BitOr, Not};

#[derive(Debug, Clone)]
pub struct BoolOp {
    pub inside: Vec<Vec<bool>>,
}

impl BoolOp {
    pub fn new(mask: Vec<Vec<bool>>) -> BoolOp {
        BoolOp { inside: mask }
    }
}

impl BitAnd for BoolOp {
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

impl BitOr for BoolOp {
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

impl Not for BoolOp {
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
    pub intersections: Vec<Point>,

    built: bool,
}

/// BeamTable acceleration structure. Creates a geometric space lookup table.
impl BeamTable {
    pub fn new(geometry: Geomstr) -> BeamTable {
        BeamTable {
            geometry,
            events: Vec::new(),
            actives: Vec::new(),
            intersections: Vec::new(),
            built: false,
        }
    }

    /// Create an Even/Odd fill for a given layer level.
    pub fn evenodd_fill(&self, layer: f64) -> BoolOp {
        let mut spacemask = Vec::new();
        for active in &self.actives {
            let mut active_mask = Vec::new();
            let mut inside = false;
            active_mask.push(inside);
            for a in active {
                let line = &self.geometry.segments[*a as usize];
                if line.2 .1 == layer {
                    inside = !inside;
                }
                active_mask.push(inside);
            }
            spacemask.push(active_mask);
        }
        BoolOp::new(spacemask)
    }

    /// Create an even_odd fill for all geometry.
    /// Useful for point in polygon solutions
    pub fn even_odd_ignoring_origin(&self) -> BoolOp {
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
        BoolOp::new(spacemask)
    }

    /// Create a union of all layers
    pub fn union_all(&self) -> BoolOp {
        let mut spacemask = Vec::new();
        for active in &self.actives {
            let mut set: HashMap<i32, bool> = HashMap::new();
            let mut active_mask = Vec::new();
            active_mask.push(set.len() != 0);
            for a in active {
                let line = &self.geometry.segments[*a as usize];
                if set.contains_key(&(line.2 .1 as i32)) {
                    set.remove(&(line.2 .1 as i32));
                } else {
                    set.insert(line.2 .1 as i32, true);
                }
                active_mask.push(set.len() != 0);
            }
            spacemask.push(active_mask);
        }
        BoolOp::new(spacemask)
    }

    /// Create geometry from a BoolOp.
    pub fn create(&self, mask: BoolOp) -> Geomstr {
        let mut g = Geomstr::new();
        let inside = &mask.inside;
        for j in 0..inside.len() - 1 {
            //mask exists at inside-1, but the final entry is actually pointless
            let left_event = &self.events[j];
            let beam_active = &self.actives[j];
            let right_event = &self.events[j + 1];

            for k in 0..inside[j].len() - 1 {
                let below_space = inside[j][k];
                let segment_active = beam_active[k];
                let above_space = inside[j][k + 1];
                if (below_space && !above_space) || (!below_space && above_space) {
                    //is a boundary.
                    let start = self.geometry.y_intercept(
                        segment_active as usize,
                        left_event.x,
                        left_event.y,
                    );
                    let end = self.geometry.y_intercept(
                        segment_active as usize,
                        right_event.x,
                        right_event.y,
                    );
                    let line = &self.geometry.segments[segment_active as usize];
                    g.line((start.x, start.y), (end.x, end.y), line.2 .1);
                }
            }
        }
        g
    }

    /// Find the actives for a particlar x/y event space.
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

    /// Internal: find the position within the given actives for the current x.
    fn bisect_yints(&self, actives: &Vec<i32>, x: i32, scanline: &Point) -> i32 {
        let geometry = &self.geometry;
        let mut lo = 0;
        let mut hi = actives.len();
        let mut mid;
        while lo < hi {
            mid = (lo + hi) / 2;
            let test = &geometry.y_intercept(actives[mid] as usize, scanline.x, scanline.y);
            let value = &geometry.y_intercept(x as usize, scanline.x, scanline.y);
            match Point::cmp(&value, &test) {
                Ordering::Less => {
                    hi = mid;
                }
                Ordering::Greater => {
                    lo = mid + 1;
                }
                Ordering::Equal => {
                    let test_slope = &geometry.slope(actives[mid] as usize);
                    let value_slope = &geometry.slope(x as usize);
                    if value_slope < test_slope {
                        hi = mid
                    } else {
                        lo = mid + 1
                    }
                }
            }
        }
        lo as i32
    }

    /// Internal: check for intersections between indexes q and r, occurring after sl
    fn check_intersections(
        &mut self,
        events: &mut BinaryHeap<Event>,
        actives: &Vec<i32>,
        checked_swaps: &mut Vec<(i32, i32)>,
        q: usize,
        r: usize,
        sl: &Point,
    ) {
        let q = actives[q];
        let r = actives[r];
        let geometry = &self.geometry;
        if checked_swaps.contains(&(q, r)) {
            return;
        }
        let intersection = geometry.get_intersection(q as usize, r as usize);

        match intersection {
            None => (),
            Some(t) => {
                let t1 = t.0;
                let t2 = t.1;
                if (t1 == 0.0 || t1 == 1.0) && ((t2 == 0.0) || (t2 == 1.0)) {
                    return;
                }
                let pt_intersect = geometry.point(q as usize, t1);
                self.intersections.push(pt_intersect.clone());
                match Point::cmp(&sl, &pt_intersect) {
                    Ordering::Greater => {
                        return;
                    }
                    Ordering::Equal => {
                        return;
                    }
                    Ordering::Less => {}
                }
                checked_swaps.push((q, r));
                let event = Event {
                    point: pt_intersect,
                    index: 0,
                    swap: Some((q, r)),
                };
                events.push(event);
            }
        }
    }

    /// Builds the beamtable from the underlying geometry.
    pub fn build(&mut self) {
        if self.built {
            //This was already built.
            return;
        }
        let mut events: BinaryHeap<Event> = BinaryHeap::new();
        let mut checked_swaps: Vec<(i32, i32)> = Vec::new();
        let mut actives: Vec<i32> = Vec::new();

        // Create initial start and end values for the event queue.
        for i in 0..self.geometry.segments.len() {
            let line = &self.geometry.segments[i];
            let p0 = Point::new(line.0 .0, line.0 .1);
            let p1 = Point::new(line.4 .0, line.4 .1);
            match Point::cmp(&p0, &p1) {
                Ordering::Less => {
                    events.push(Event {
                        point: p0,
                        index: i as i32,
                        swap: None,
                    });
                    events.push(Event {
                        point: p1,
                        index: !i as i32,
                        swap: None,
                    });
                }
                _ => {
                    events.push(Event {
                        point: p1,
                        index: i as i32,
                        swap: None,
                    });
                    events.push(Event {
                        point: p0,
                        index: !i as i32,
                        swap: None,
                    });
                }
            }
        }

        // Process the event queue, performs Bentley-Ottmann line intersection checks
        while events.len() != 0 {
            let event = events
                .pop()
                .expect("Pop only called after checking events existed.");
            let idx = event.index;
            let index = event.index;
            let pt = &event.point;
            match event.swap {
                None => {
                    if idx >= 0 {
                        // Insert.
                        let ip = self.bisect_yints(&actives, index, &event.point) as usize;
                        actives.insert(ip, index);
                        if ip > 0 {
                            self.check_intersections(
                                &mut events,
                                &actives,
                                &mut checked_swaps,
                                ip - 1,
                                ip,
                                pt,
                            )
                        }
                        if ip < actives.len() - 1 {
                            self.check_intersections(
                                &mut events,
                                &actives,
                                &mut checked_swaps,
                                ip,
                                ip + 1,
                                pt,
                            )
                        }
                    } else {
                        //Remove.
                        let rp = actives
                            .iter()
                            .position(|&e| e == !index)
                            .expect("Was added should remove.");
                        actives.remove(rp);
                        if 0 < rp && rp < actives.len() {
                            self.check_intersections(
                                &mut events,
                                &actives,
                                &mut checked_swaps,
                                rp - 1,
                                rp,
                                pt,
                            )
                        }
                    }
                }
                Some((s1, _)) => {
                    let s1 = actives
                        .iter()
                        .position(|&e| e == s1)
                        .expect("Swap pos should exist.");
                    let s2 = s1 + 1;
                    actives.swap(s1, s2);
                    if s1 > 0 {
                        self.check_intersections(
                            &mut events,
                            &actives,
                            &mut checked_swaps,
                            s1 - 1,
                            s1,
                            pt,
                        );
                    }
                    if s2 < actives.len() - 1 {
                        self.check_intersections(
                            &mut events,
                            &actives,
                            &mut checked_swaps,
                            s2,
                            s2 + 1,
                            pt,
                        );
                    }
                }
            }
            match events.peek() {
                None => {}
                Some(last_pt) => {
                    if pt == &last_pt.point {
                        continue;
                    }
                }
            }

            // Push the current state to the table
            self.events.push((*pt).clone());
            self.actives.push(actives.clone());
        }
        self.built = true;
    }
}

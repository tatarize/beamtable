use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::events::Event;
use crate::geometry::{Geomstr,  Point};
use crate::table::BeamTable;

pub struct ScanBeam {
    pub segments: Geomstr,
    pub intersections: Vec<Point>,
    pub events: BinaryHeap<Event>,
    pub actives: Vec<i32>,
    checked_swaps: Vec<(i32, i32)>,
}

impl ScanBeam {
    pub fn new(segments: Geomstr) -> ScanBeam {
        ScanBeam {
            segments,
            events: BinaryHeap::new(),
            actives: Vec::new(),
            intersections: Vec::new(),
            checked_swaps: Vec::new(),
        }
    }

    /// Find the position within the actives for the current x.
    fn bisect_yints(&self, x: i32, scanline: &Point) -> i32 {
        let actives = &self.actives;
        let segments = &self.segments;
        let mut lo = 0;
        let mut hi = actives.len();
        let mut mid;
        while lo < hi {
            mid = (lo + hi) / 2;
            let test = &segments.y_intercept(actives[mid] as usize, scanline.x, scanline.y);
            let value = &segments.y_intercept(x as usize, scanline.x, scanline.y);
            match Point::cmp(&value, &test) {
                Ordering::Less => {
                    hi = mid;
                }
                Ordering::Greater => {
                    lo = mid + 1;
                }
                Ordering::Equal => {
                    let test_slope = &segments.slope(actives[mid] as usize);
                    let value_slope = &segments.slope(x as usize);
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

    fn check_intersections(&mut self, q: usize, r: usize, sl: &Point) {
        let actives = &self.actives;
        let q = actives[q];
        let r = actives[r];
        let segments = &self.segments;
        let checked_swaps = &mut self.checked_swaps;
        // println!("{q} {r}");
        if checked_swaps.contains(&(q, r)) {
            return;
        }
        let intersection = segments.get_intersection(q as usize, r as usize);

        match intersection {
            None => (),
            Some(t) => {
                let t1 = t.0;
                let t2 = t.1;
                // println!("{t1}, {t2}");
                if ((t1 == 0.0 || t1 == 1.0)) && ((t2 == 0.0) || (t2 == 1.0)) {
                    return;
                }
                let pt_intersect = segments.point(q as usize, t1);
                self.intersections.push(pt_intersect.clone());
                match Point::cmp(&sl, &pt_intersect) {
                    Ordering::Greater => { return; }
                    Ordering::Equal => { return; }
                    Ordering::Less => {}
                }
                checked_swaps.push((q, r));
                let event = Event {
                    point: pt_intersect,
                    index: 0,
                    swap: Some((q, r)),
                };
                self.events.push(event);
            }
        }
    }

    pub fn build(&mut self) -> BeamTable {
        let events = &mut self.events;
        let segments = &self.segments;
        for i in 0..segments.segments.len() {
            let line = &self.segments.segments[i];
            let p0 = Point::new(line.0.0, line.0.1);
            let p1 = Point::new(line.4.0, line.4.1);
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
        // Sort the events start and end.
        // events.sort();

        let mut bt = BeamTable::new();
        // println!("{:?}", events);
        while self.events.len() != 0 {
            let event = self.events.pop().expect("Pop only called after checking events existed.");
            let idx = event.index;
            let index = event.index;
            let pt = &event.point;
            // println!("{:?}", event);
            match event.swap {
                None => {
                    if idx >= 0 {
                        // Insert.
                        let ip = self.bisect_yints(index, &event.point) as usize;
                        self.actives.insert(ip, index);
                        if ip > 0 {
                            self.check_intersections(ip - 1, ip, pt)
                        }
                        if ip < self.actives.len() - 1 {
                            self.check_intersections(ip, ip + 1, pt)
                        }
                    } else {
                        //Remove.
                        let rp = self.actives.iter().position(|&e| e == !index).expect("Was added should remove.");
                        self.actives.remove(rp);
                        if 0 < rp && rp < self.actives.len() {
                            self.check_intersections(rp - 1, rp, pt)
                        }
                    }
                }
                Some((s1, _)) => {
                    let s1 = self.actives.iter().position(|&e| e == s1).expect("Swap pos should exist.");
                    let s2 = s1 + 1;
                    self.actives.swap(s1, s2);
                    if s1 > 0 {
                        self.check_intersections(s1 - 1, s1, pt);
                    }
                    if s2 < self.actives.len() - 1 {
                        self.check_intersections(s2, s2 + 1, pt);
                    }
                }
            }
            match self.events.peek() {
                None => { }
                Some(last_pt) => {
                    if pt == &last_pt.point {
                        continue
                    }
                }
            }

            bt.events.push((*pt).clone());
            bt.actives.push(self.actives.clone());
        }
        bt
    }
}

use std::cmp::Ordering;

use crate::geometry::Point;

/// Events for the scantable build. Each event is sorted by x, y, and slope in that order.
/// Ordering is done in *reversed* order to make the BinaryHeap structure give a minheap.
#[derive(Debug, Clone)]
pub struct Event {
    pub point: Point,
    pub add: Vec<usize>,
    pub remove: Vec<usize>,
    pub update: Vec<usize>,
}

impl Event {
    pub fn new(x: f64, y: f64) -> Event {
        Event {
            point: Point::new(x, y),
            add: Vec::new(),
            remove: Vec::new(),
            update: Vec::new(),
        }
    }

    pub fn swap(pt: Point, s1: usize, s2: usize) -> Event {
        Event {
            point: pt,
            add: Vec::new(),
            remove: Vec::new(),
            update: vec![s1, s2],
        }
    }
    pub fn start(pt: Point, start: usize) -> Event {
        Event {
            point: pt,
            add: vec![start],
            remove: Vec::new(),
            update: Vec::new(),
        }
    }
    pub fn end(pt: Point, end: usize) -> Event {
        Event {
            point: pt,
            add: Vec::new(),
            remove: vec![end],
            update: Vec::new(),
        }
    }
}

impl Eq for Event {}

impl PartialEq<Self> for Event {
    fn eq(&self, other: &Self) -> bool {
        return self.point.x == other.point.x && self.point.y == other.point.y;
    }
}

impl PartialOrd<Self> for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        match Point::cmp(&self.point, &other.point) {
            Ordering::Greater => {
                return Ordering::Less;
            }
            Ordering::Less => {
                return Ordering::Greater;
            }
            Ordering::Equal => {
                return Ordering::Equal;
            }
        }
    }
}

use std::cmp::Ordering;

use crate::geometry::Point;

#[derive(Debug, Clone)]
pub struct Event {
    pub point: Point,
    pub index: i32,
    pub swap: Option<(i32, i32)>,
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
            Ordering::Greater => { return Ordering::Less; }
            Ordering::Less => { return Ordering::Greater; }
            Ordering::Equal => {}
        }
        if other.index > self.index {
            return Ordering::Less;
        }
        if other.index < self.index {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}


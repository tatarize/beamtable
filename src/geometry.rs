use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl Eq for Point {}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.x < self.x {
            return Ordering::Greater;
        } else if other.x > self.x {
            return Ordering::Less;
        }
        if other.y < self.y {
            return Ordering::Greater;
        } else if other.y > self.y {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}


#[derive(Debug, Clone)]
pub struct Line {
    /// The line's start point.
    pub p0: Point,
    /// The line's end point.
    pub p1: Point,

    pub index: usize,
}

impl Line {
    /// Create a new line.
    #[inline]
    pub fn new(p0: impl Into<Point>, p1: impl Into<Point>, index: usize) -> Line {
        Line {
            p0: p0.into(),
            p1: p1.into(),
            index,
        }
    }

    /// Slope where divide by 0 is always negative infinity.
    pub fn slope(&self) -> f64 {
        let rise: f64 = self.p0.y - self.p1.y;
        let run: f64 = self.p0.x - self.p1.x;
        if run == 0.0 {
            return f64::NEG_INFINITY;
        }
        rise / run
    }

    pub fn get_intersection(&self, line: &Line) -> Option<(f64, f64)> {
        let a = &self.p0;
        let b = &self.p1;
        let c = &line.p0;
        let d = &line.p1;
        let denom: f64 = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);
        if denom.abs() < 1e-12 { return None; }
        let t1: f64 = ((d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x)) / denom;
        let t2: f64 = ((b.x - a.x) * (a.y - c.y) - (b.y - a.y) * (a.x - c.x)) / denom;
        if 0.0 <= t1 && t1 <= 1.0 && 0.0 <= t2 && t2 <= 1.0 {
            return Some((t1, t2));
        }
        None
    }

    ///Returns the y_intercept point given a line a given x. Default is used for y if there is a line along the requested x
    pub fn y_intercept(&self, x: f64, default: f64) -> Point {
        let a = &self.p0;
        let b = &self.p1;
        let rise: f64 = a.y - b.y;
        let run: f64 = a.x - b.x;
        if rise == 0.0 {
            return Point::new(x, a.y);
        }
        if run == 0.0 {
            return Point::new(x, default);
        }
        let m = run / rise;
        let x0: f64 = a.x - (m * a.y);
        Point::new(x, (x - x0) / m)
    }

    pub fn point(&self, t: f64) -> Point {
        Point::new(t * (self.p1.x - self.p0.x) + self.p0.x, t * (self.p1.y - self.p0.y) + self.p0.y)
    }
}



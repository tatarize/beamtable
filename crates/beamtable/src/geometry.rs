use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

///Standard x,y point. Sort order for the point is x with tie breaks going to higher y-value.
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl Eq for Point {}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        return (self.x - other.x).abs() < 1e-12 && (self.y - other.y).abs() < 1e-12;
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let x_eq = (self.x - other.x).abs() < 1e-12;
        if x_eq {
            if (self.y - other.y).abs() < 1e-12 {
                return Ordering::Equal;
            }
            return f64::partial_cmp(&self.y, &other.y).expect("No NaNs");
        }
        f64::partial_cmp(&self.x, &other.x).expect("No NaNs")
    }
}

/// Geomstr: Geometry class see, sister structure:
/// https://github.com/meerk40t/meerk40t/blob/main/meerk40t/tools/geomstr.py
#[derive(Debug, Clone)]
pub struct Geomstr {
    pub segments: Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64))>,
}

impl Geomstr {
    pub fn new() -> Geomstr {
        Geomstr {
            segments: Vec::new(),
        }
    }
    pub fn from_segments(
        segments: Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64))>,
    ) -> Geomstr {
        Geomstr { segments }
    }

    /// Add a rectangle to the geometry.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, settings: f64) {
        self.line((x, y), (x + width, y), settings);
        self.line((x + width, y), (x + width, y + height), settings);
        self.line((x + width, y + height), (x, y + height), settings);
        self.line((x, y + height), (x, y), settings);
    }

    /// Add a line to the geometry.
    pub fn line(&mut self, p0: (f64, f64), p1: (f64, f64), settings: f64) {
        self.segments
            .push((p0, (0., 0.), (41.0, settings), (0., 0.), p1));
    }

    /// Slope where divide by 0 is always negative infinity.
    pub fn slope(&self, index: usize) -> f64 {
        let line = &self.segments[index];
        let rise: f64 = line.0 .1 - line.4 .1;
        let run: f64 = line.0 .0 - line.4 .0;
        if run == 0.0 {
            return f64::INFINITY;
        }
        rise / run
    }

    /// Find an intersection between index0 and index1.
    pub fn get_intersection(&self, index0: usize, index1: usize) -> Option<(f64, f64)> {
        let line0 = &self.segments[index0];
        let line1 = &self.segments[index1];
        let a = &line0.0;
        let b = &line0.4;
        let c = &line1.0;
        let d = &line1.4;
        let denom: f64 = (d.1 - c.1) * (b.0 - a.0) - (d.0 - c.0) * (b.1 - a.1);
        if denom.abs() < 1e-12 {
            return None;
        }
        let t1: f64 = ((d.0 - c.0) * (a.1 - c.1) - (d.1 - c.1) * (a.0 - c.0)) / denom;
        let t2: f64 = ((b.0 - a.0) * (a.1 - c.1) - (b.1 - a.1) * (a.0 - c.0)) / denom;
        if 0.0 <= t1 && t1 <= 1.0 && 0.0 <= t2 && t2 <= 1.0 {
            return Some((t1, t2));
        }
        None
    }

    /// Returns the y_intercept point given a line a given x.
    /// Default is used for y if there is a line along the requested x.
    pub fn y_intercept(&self, index: usize, x: f64, default: f64) -> Point {
        let line = &self.segments[index];
        let a = &line.0;
        let b = &line.4;
        let rise: f64 = a.1 - b.1;
        let run: f64 = a.0 - b.0;
        if rise == 0.0 {
            return Point::new(x, a.1);
        }
        if run == 0.0 {
            return Point::new(x, default);
        }
        let m = run / rise;
        let x0: f64 = a.0 - (m * a.1);
        Point::new(x, (x - x0) / m)
    }

    /// Find point located within the current geometry at position t [0,1]
    pub fn point(&self, index: usize, t: f64) -> Point {
        let line = &self.segments[index];
        Point::new(
            t * (line.4 .0 - line.0 .0) + line.0 .0,
            t * (line.4 .1 - line.0 .1) + line.0 .1,
        )
    }

    ///Check overall string distances
    pub fn travel_distance_sq(&self) -> f64{
        let mut total = 0.0;
        for i in 1..self.segments.len() {
            let line1 = &self.segments[i-1];
            let line2 = &self.segments[i];
            if line1.2.0 != 41.0 {
                continue;
            }
            if line2.2.0 != 41.0 {
                continue;
            }
            let dx = line1.4.0 - line2.0.0;
            let dy = line1.4.1 - line2.0.1;
            let delta = dx * dx + dy * dy;
            total += delta
        }
        total
    }

    pub fn reverse(&mut self, element: usize) {
        let g = &self.segments[element];
        self.segments[element] = ((g.4.0, g.4.1), (g.3.0, g.3.1), (g.2.0, g.2.1), (g.1.0, g.1.1), (g.0.0, g.0.1))
    }

    /// Perform greedy optimization to minimize travel distances
    pub fn greedy_distance(&mut self, mut pt: Point, flips: bool) {
        for j in 0..self.segments.len() {
            let mut best = f64::INFINITY;
            let mut best_k = usize::MAX;
            let mut best_flip = false;
            if j > 0 {
                pt.x = self.segments[j-1].4.0;
                pt.y = self.segments[j-1].4.1;
            }

            for k in (j+1)..self.segments.len() {
                let kline = &self.segments[k-1];
                if flips {
                    let dx = pt.x - kline.4.0;
                    let dy = pt.y - kline.4.1;
                    let delta = dx * dx + dy * dy;
                    if delta < best {
                        best = delta;
                        best_k = k-1;
                        best_flip = true;
                    }
                }
                let dx = pt.x - kline.0.0;
                let dy = pt.y - kline.0.1;
                let delta = dx * dx + dy * dy;
                if delta < best {
                    best = delta;
                    best_k = k-1;
                    best_flip = false
                }
            }
            if best_k != usize::MAX {
                if best_flip { self.reverse(best_k); }
                self.segments.swap(j, best_k);
            }
        }
    }
}

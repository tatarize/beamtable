#[cfg(test)]
mod tests {
    use rand::prelude::ThreadRng;
    use rand::Rng;
    use crate::geometry::{Line, Point};
    use crate::scanbeam::ScanBeam;

    #[test]
    fn two_box_test() {
        let segments = vec![
            Line::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0), 1),
            Line::new(Point::new(100.0, 0.0), Point::new(100.0, 100.0), 1),
            Line::new(Point::new(100.0, 100.0),Point::new(0.0, 100.0), 1),
            Line::new(Point::new(0.0, 100.0),Point::new(0.0, 0.0), 1),

            Line::new(Point::new(5.0, 5.0), Point::new(5.0, 105.0), 0),
            Line::new(Point::new(5.0, 105.0), Point::new(105.0, 105.0), 0),
            Line::new(Point::new(105.0, 105.0),Point::new(105.0, 5.0), 0),
            Line::new(Point::new(105.0, 5.0),Point::new(5.0, 5.0), 0)
        ];

        let mut table = ScanBeam::new(segments);
        let _q = table.build();
        // println!("{:?}", q.actives);
        // println!("{:?}", q.events);
    }

    #[test]
    fn actives_monotonic() {
        let mut segments: Vec<Line> = Vec::new();
        {
            let mut rng = ThreadRng::default();
            for c in 0..10 {
                for _i in 0..25 {
                    segments.push(
                        Line::new(
                            Point::new(rng.gen_range(0..1000) as f64, rng.gen_range(0..1000) as f64),
                            Point::new(rng.gen_range(0..1000) as f64, rng.gen_range(0..1000) as f64),
                            c
                        ))
                }
            }
        }
        let mut beam = ScanBeam::new(segments);
        let table = beam.build();
        // println!("{:?}", table.actives);
        // println!("{:?}", table.events);
        for x in 0..1000 {
            let x = x as f64;
            let actives = table.actives_at(x, 0.0);
            for i in 1..actives.len() {
                let prev = &beam.segments[actives[i -1] as usize];
                let line = &beam.segments[actives[i] as usize];
                let pp0 = (&prev.p0).x;
                let pp1 = (&prev.p1).x;
                if &prev.p0.x < &prev.p1.x {
                    // println!("{pp0:?} {pp1:?} for {x:?}");
                    assert!(x >= pp0);
                    assert!(x <= pp1);
                }
                else {
                    // println!("{pp1:?} {pp0:?} for {x:?}");
                    assert!(x >= pp1);
                    assert!(x <= pp0);
                }
                let last_pos = prev.y_intercept(x, 0.0);
                let pos = line.y_intercept(x, 0.0);
                // println!("{last_pos:?} {pos:?}");
                assert!(last_pos <= pos);
            }
        }
    }

}
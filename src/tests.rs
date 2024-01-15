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
        let q = table.build();
        println!("{:?}", q.actives);
        println!("{:?}", q.events);
        // assert_eq!(result, 4);
    }

    #[test]
    fn actives_monotonic() {
        let mut segments: Vec<Line> = Vec::new();
        for _c in 0..10 {
            for _i in 0..25 {
                let mut rng = ThreadRng::default();

                segments.push(
                    Line::new(
                        Point::new(rng.gen_range(0..1000) as f64, rng.gen_range(0..1000) as f64),
                        Point::new(rng.gen_range(0..1000) as f64, rng.gen_range(0..1000) as f64),
                        0
                    ))
            }
        }
        let mut beam = ScanBeam::new(segments);
        let table = beam.build();
        for x in 0..1000 {
            let actives = table.actives_at(x as f64, 0.0);
            for active in 1..(actives.len() - 1) {
                let prev = &beam.segments[active-1];
                let line = &beam.segments[active];
                let last_pos = prev.y_intercept(x as f64, 0.0);
                let pos = line.y_intercept(x as f64, 0.0);
                println!("{last_pos:?} {pos:?}");
                assert!(last_pos >= pos);
            }
        }
    }

}
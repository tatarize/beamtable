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
                if pp0 < pp1 {
                    // println!("{pp0:?} {pp1:?} for {x:?}");
                    assert!(x >= pp0);
                    assert!(x <= pp1);
                }
                else {
                    // println!("{pp1:?} {pp0:?} for {x:?}");
                    assert!(x >= pp1);
                    assert!(x <= pp0);
                }
                let cp0 = (&line.p0).x;
                let cp1 = (&line.p1).x;
                if cp0 < cp1 {
                    // println!("{cp0:?} {cp1:?} for {x:?}");
                    assert!(x >= cp0);
                    assert!(x <= cp1);
                }
                else {
                    // println!("{cp1:?} {cp0:?} for {x:?}");
                    assert!(x >= cp1);
                    assert!(x <= cp0);
                }
                let last_pos = prev.y_intercept(x, 0.0);
                let pos = line.y_intercept(x, 0.0);
                // println!("{last_pos:?} {pos:?}");
                assert!(last_pos <= pos);
            }
        }
    }

    #[test]
    fn actives_float_rounding() {
        let segments= vec![
            Line { p0: Point { x: 680.0, y: 134.0 }, p1: Point { x: 725.0, y: 509.0 }, index: 2 },
            Line { p0: Point { x: 937.0, y: 186.0 }, p1: Point { x: 228.0, y: 243.0 }, index: 8 },
            Line { p0: Point { x: 746.0, y: 867.0 }, p1: Point { x: 680.0, y: 52.0 }, index: 9 },
            Line { p0: Point { x: 961.0, y: 481.0 }, p1: Point { x: 662.0, y: 182.0 }, index: 7 }
        ];
        let mut beam = ScanBeam::new(segments);
        let table = beam.build();
        println!("{:?}", table.actives);
        println!("{:?}", table.events);
        for x in 689..690 {
            let x = x as f64;
            let actives = table.actives_at(x, 0.0);
            for i in 1..actives.len() {
                let prev = &beam.segments[actives[i -1] as usize];
                let line = &beam.segments[actives[i] as usize];
                let pp0 = (&prev.p0).x;
                let pp1 = (&prev.p1).x;
                if pp0 < pp1 {
                    println!("{pp0:?} {pp1:?} for {x:?}");
                    assert!(x >= pp0);
                    assert!(x <= pp1);
                }
                else {
                    println!("{pp1:?} {pp0:?} for {x:?}");
                    assert!(x >= pp1);
                    assert!(x <= pp0);
                }
                let cp0 = (&line.p0).x;
                let cp1 = (&line.p1).x;
                if cp0 < cp1 {
                    println!("{cp0:?} {cp1:?} for {x:?}");
                    assert!(x >= cp0);
                    assert!(x <= cp1);
                }
                else {
                    println!("{cp1:?} {cp0:?} for {x:?}");
                    assert!(x >= cp1);
                    assert!(x <= cp0);
                }
                let last_pos = prev.y_intercept(x, 0.0);
                let pos = line.y_intercept(x, 0.0);
                println!("{last_pos:?} {pos:?}");
                assert!(last_pos <= pos);
            }
        }
    }


}
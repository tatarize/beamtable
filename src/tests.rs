#[cfg(test)]
mod tests {
    use crate::geometry::Geomstr;
    use crate::scanbeam::ScanBeam;
    use rand::prelude::ThreadRng;
    use rand::Rng;

    #[test]
    fn two_box_test() {
        let mut segments = Geomstr::new();
        segments.rect(0.0, 0.0, 100.0, 100.0, 1.0);
        segments.rect(5.0, 5.0, 105.0, 105.0, 0.0);
        let mut table = ScanBeam::new(segments);
        let _q = table.build();
        println!("{:?}", _q.actives);
        println!("{:?}", _q.events);
    }

    #[test]
    fn actives_monotonic() {
        let mut segments = Geomstr::new();
        {
            let mut rng = ThreadRng::default();
            for c in 0..10 {
                for _i in 0..25 {
                    segments.line(
                        (rng.gen_range(0..1000) as f64, rng.gen_range(0..1000) as f64),
                        (rng.gen_range(0..1000) as f64, rng.gen_range(0..1000) as f64),
                        c as f64,
                    )
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
                let prev = &beam.segments.segments[actives[i - 1] as usize];
                let line = &beam.segments.segments[actives[i] as usize];
                let pp0 = prev.0 .0;
                let pp1 = prev.4 .0;
                if pp0 < pp1 {
                    // println!("{pp0:?} {pp1:?} for {x:?}");
                    assert!(x >= pp0);
                    assert!(x <= pp1);
                } else {
                    // println!("{pp1:?} {pp0:?} for {x:?}");
                    assert!(x >= pp1);
                    assert!(x <= pp0);
                }
                let cp0 = line.0 .0;
                let cp1 = line.4 .0;
                if cp0 < cp1 {
                    // println!("{cp0:?} {cp1:?} for {x:?}");
                    assert!(x >= cp0);
                    assert!(x <= cp1);
                } else {
                    // println!("{cp1:?} {cp0:?} for {x:?}");
                    assert!(x >= cp1);
                    assert!(x <= cp0);
                }
                let last_pos = &beam.segments.y_intercept(actives[i - 1] as usize, x, 0.0);
                let pos = &beam.segments.y_intercept(actives[i] as usize, x, 0.0);
                // println!("{last_pos:?} {pos:?}");
                assert!(last_pos <= pos);
            }
        }
    }

    #[test]
    fn actives_float_rounding() {
        let mut segments = Geomstr::new();
        segments.line((680.0, 134.0), (725.0, 509.0), 2.0);
        segments.line((937.0, 186.0), (228.0, 243.0), 8.0);
        segments.line((746.0, 867.0), (680.0, 52.0), 9.0);
        segments.line((961.0, 481.0), (662.0, 182.0), 7.0);

        let mut beam = ScanBeam::new(segments);
        let table = beam.build();
        println!("{:?}", table.actives);
        println!("{:?}", table.events);
        for x in 689..690 {
            let x = x as f64;
            let actives = table.actives_at(x, 0.0);
            for i in 1..actives.len() {
                let prev = &beam.segments.segments[actives[i - 1] as usize];
                let line = &beam.segments.segments[actives[i] as usize];
                let pp0 = prev.0 .0;
                let pp1 = prev.4 .0;
                if pp0 < pp1 {
                    println!("{pp0:?} {pp1:?} for {x:?}");
                    assert!(x >= pp0);
                    assert!(x <= pp1);
                } else {
                    println!("{pp1:?} {pp0:?} for {x:?}");
                    assert!(x >= pp1);
                    assert!(x <= pp0);
                }
                let cp0 = line.0 .0;
                let cp1 = line.4 .0;
                if cp0 < cp1 {
                    println!("{cp0:?} {cp1:?} for {x:?}");
                    assert!(x >= cp0);
                    assert!(x <= cp1);
                } else {
                    println!("{cp1:?} {cp0:?} for {x:?}");
                    assert!(x >= cp1);
                    assert!(x <= cp0);
                }
                let last_pos = &beam.segments.y_intercept(actives[i - 1] as usize, x, 0.0);
                let pos = &beam.segments.y_intercept(actives[i] as usize, x, 0.0);
                println!("{last_pos:?} {pos:?}");
                assert!(last_pos <= pos);
            }
        }
    }
}

mod geometry;
mod events;
mod scanbeam;
mod table;

use pyo3::prelude::*;
use crate::scanbeam::ScanBeam;

///
#[pyfunction]
fn build(segments: Vec<(f64, f64, f64, f64)>) -> PyResult<(Vec<(f64, f64)>, Vec<Vec<i32>>)> {
    let mut table = ScanBeam::from_floats(segments);
    let q = table.build();

    let mut segs = Vec::new();
    for m in q.events.iter() {
        segs.push((m.x, m.y));
    }
    Ok((segs, q.actives))
}

/// A Python module implemented in Rust.
#[pymodule]
fn beamtable(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build, m)?)?;
    Ok(())
}

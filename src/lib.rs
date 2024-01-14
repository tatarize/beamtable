mod geometry;
mod events;
mod scanbeam;
mod table;

use pyo3::prelude::*;
use crate::scanbeam::ScanBeam;

///
#[pyfunction]
fn build(segments: Vec<(f64, f64, f64, f64)>) -> PyResult<(Vec<(f64, f64)>, Vec<Vec<i32>>)> {
    let mut table = ScanBeam::new(segments);
    let q = table.build();
    Ok(q.events, q.actives)
}

/// A Python module implemented in Rust.
#[pymodule]
fn beamtable(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build, m)?)?;
    Ok(())
}

mod events;
pub mod geometry;
pub mod table;
mod tests;

use crate::geometry::Geomstr;
use crate::table::BeamTable;
use pyo3::prelude::*;

#[pyfunction]
fn union(
    segments: Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64))>,
) -> Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64))> {
    let mut table = BeamTable::new(Geomstr::from_segments(segments));
    table.build();
    let bo = table.union_all();
    table.create(bo).segments
}

/// A Python module implemented in Rust.
#[pymodule]
fn beamtable(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(union, m)?)?;
    Ok(())
}

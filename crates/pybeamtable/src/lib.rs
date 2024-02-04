use ::beamtable::{BeamTable, Geomstr};
use pyo3::prelude::*;

// #[pyclass]
// struct BeamTable {
//     #[pyo3(get)]
//     pub events: Vec<PyComplex>,
//     #[pyo3(get)]
//     pub actives: Vec<Vec<i32>>,
// }
//
// #[pymethods]
// impl BeamTable {
//     #[new]
//     pub fn new() -> BeamTable {
//         BeamTable {
//             events: Vec::new(),
//             actives: Vec::new(),
//         }
//     }
// }

///
// #[pyfunction]
// fn build(segments: Vec<(f64, f64, f64, f64)>) -> PyResult<(Vec<(f64, f64)>, Vec<Vec<i32>>)> {
//     let mut table = ScanBeam::from_floats(segments);
//     let q = table.build();
//
//     let mut segs = Vec::new();
//     for m in q.events.iter() {
//         segs.push((m.x, m.y));
//     }
//     Ok((segs, q.actives))
// }
#[pyfunction]
fn union(
    segments: Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64))>,
) -> Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64))> {
    let mut table = BeamTable::new(Geomstr::from_segments(segments));
    table.build();
    let bo = table.union_all();
    table.create(bo, true).segments
}

/// A Python module implemented in Rust.
#[pymodule]
fn pybeamtable(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(union, m)?)?;
    // m.add_function(wrap_pyfunction!(complex_test, m)?)?;
    // m.add_class::<BeamTable>()?;
    Ok(())
}

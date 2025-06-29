pub mod packet;
mod crc;

use pyo3::prelude::*;

/// python module
#[pymodule]
fn proto170(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(packet::build_request, m)?)?;
    Ok(())
}

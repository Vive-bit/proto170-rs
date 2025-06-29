mod protocol;
mod util;
mod crc;

use pyo3::prelude::*;
use protocol::*;

#[pymodule]
fn proto170(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_request, m)?)?;
    m.add_function(wrap_pyfunction!(build_ping_request, m)?)?;
    m.add_function(wrap_pyfunction!(build_register_request, m)?)?;
    m.add_function(wrap_pyfunction!(build_read_request, m)?)?;
    Ok(())
}

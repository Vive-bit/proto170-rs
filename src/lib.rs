mod protocol;
mod util;
mod crc;

use pyo3::prelude::*;
use protocol::*;

#[pymodule]
fn proto170(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PacketConstants>()?;
    m.add_function(wrap_pyfunction!(build_request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(build_response_frame, m)?)?;
    m.add_function(wrap_pyfunction!(check_crc, m)?)?;
    Ok(())
}

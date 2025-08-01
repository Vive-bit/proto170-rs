mod protocol;
mod util;
mod crc;

use pyo3::prelude::*;
use protocol::*;
use util::*;

#[pymodule]
fn proto170(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PacketConstants>()?;
    m.add_function(wrap_pyfunction!(build_request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(build_frame, m)?)?;
    m.add_function(wrap_pyfunction!(check_crc, m)?)?;
    m.add_function(wrap_pyfunction!(pack, m)?)?;
    m.add_function(wrap_pyfunction!(unpack, m)?)?;
    Ok(())
}

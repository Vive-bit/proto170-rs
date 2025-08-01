mod protocol;
mod util;
mod crc;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use protocol::{build_request_frame, build_frame, check_crc, PacketConstants};
use util::{pack, unpack};

#[pymodule]
fn proto170(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PacketConstants>()?;
    m.add_function(wrap_pyfunction!(build_request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(build_frame, m)?)?;
    m.add_function(wrap_pyfunction!(check_crc, m)?)?;
    m.add_function(wrap_pyfunction!(pack, m)?)?;
    m.add_function(wrap_pyfunction!(unpack, m)?)?;
    Ok(())
}

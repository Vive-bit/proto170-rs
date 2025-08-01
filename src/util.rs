use pyo3::prelude::*;
use pyo3::pyfunction;

#[pyfunction]
pub fn pack(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) | b as u16
}

#[pyfunction]
pub fn unpack(val: u16) -> (u8, u8) {
    ((val >> 8) as u8, (val & 0xFF) as u8)
}

use pyo3::prelude::*;
use crate::crc::crc16;

pub const START_BYTE: u8 = 0xAA;
pub const START_BYTE_MASTER: u8 = 0x7E;
pub const REQUEST_FRAME_TYPE: u8 = 0x10;
pub const RESPONSE_FRAME_TYPE: u8 = 0x20;


#[pyclass]
pub struct PacketConstants;

#[pymethods]
impl PacketConstants {
    #[classattr]
    pub const START_BYTE: u8 = START_BYTE;
    #[classattr]
    pub const START_BYTE_MASTER: u8 = START_BYTE_MASTER;
    #[classattr]
    pub const REQUEST_FRAME_TYPE: u8 = REQUEST_FRAME_TYPE;
    #[classattr]
    pub const RESPONSE_FRAME_TYPE: u8 = RESPONSE_FRAME_TYPE;
}

#[pyfunction]
pub fn build_frame(frame_type: u8, payload: Vec<u8>) -> Vec<u8> {
    let length = (payload.len() + 2) as u8;

    let mut frame = Vec::with_capacity(3 + payload.len() + 2);
    frame.push(START_BYTE);
    frame.push(frame_type);
    frame.push(length);

    frame.extend_from_slice(&payload); 

    let crc = crc16(&frame);
    frame.push((crc & 0xFF) as u8);
    frame.push((crc >> 8) as u8);

    frame
}

#[pyfunction]
pub fn build_request_frame(
    request_id: u16,
    slave_id: u8,
    request_type: u8,
    operation: u8,
    mode: u8,
    payload: Vec<u8>
) -> Vec<u8> {
    let mut frame = Vec::with_capacity(payload.len() + 6);
    frame.push((request_id >> 8) as u8);
    frame.push((request_id & 0xFF) as u8);

    frame.push(slave_id);
    frame.push(request_type);
    frame.push(operation);
    frame.push(mode);

    frame.extend_from_slice(&payload); 

    frame
}

#[pyfunction]
pub fn check_crc(data: Vec<u8>) -> bool {
    if data.len() < 3 {
        return false;
    }

    let payload = &data[..data.len() - 2];
    let crc_calc = crc16(payload);

    let crc_lsb = data[data.len() - 2];
    let crc_msb = data[data.len() - 1];

    let calc_lsb = (crc_calc & 0xFF) as u8;
    let calc_msb = (crc_calc >> 8) as u8;

    crc_lsb == calc_lsb && crc_msb == calc_msb
}

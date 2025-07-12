use pyo3::prelude::*;
use crate::crc::crc16;
use crate::util::{pack, unpack};

pub const START_BYTE: u8 = 0x7E;
pub const REQUEST_FRAME_TYPE: u8 = 0x10;
pub const RESPONSE_FRAME_TYPE: u8 = 0x20;
pub const REQUEST_FRAME_LENGTH: usize = 13;
pub const RESPONSE_FRAME_LENGTH: usize = 10;


#[pyclass]
pub struct PacketConstants;

#[pymethods]
impl PacketConstants {
    #[classattr]
    pub const START_BYTE: u8 = START_BYTE;
    #[classattr]
    pub const REQUEST_FRAME_TYPE: u8 = REQUEST_FRAME_TYPE;
    #[classattr]
    pub const RESPONSE_FRAME_TYPE: u8 = RESPONSE_FRAME_TYPE;
    #[classattr]
    pub const REQUEST_FRAME_LENGTH: usize = REQUEST_FRAME_LENGTH;
    #[classattr]
    pub const RESPONSE_FRAME_LENGTH: usize = RESPONSE_FRAME_LENGTH;
}

#[pyfunction]
pub fn build_request_frame(
    request_id: u16,
    slave_id: u8,
    request_type: u8,  // 0x01 = GET
    operation: u8,     // 0x00 = READ, 0x01 = WRITE
    mode: u8,          // 0x00 = DIGITAL, 0x01 = ANALOG
    new_state: u16,    // nur bei WRITE relevant
    gpio_id: u8,
) -> Vec<u8> {
    let mut frame = Vec::with_capacity(REQUEST_FRAME_LENGTH);
    frame.push(START_BYTE);
    frame.push(REQUEST_FRAME_TYPE);

    frame.push((request_id >> 8) as u8);
    frame.push((request_id & 0xFF) as u8);

    frame.push(slave_id);
    frame.push(request_type);
    frame.push(operation);
    frame.push(mode);

    frame.push((new_state >> 8) as u8);
    frame.push((new_state & 0xFF) as u8);

    frame.push(gpio_id);

    let crc = crc16(&frame);
    frame.push((crc & 0xFF) as u8);
    frame.push((crc >> 8) as u8);

    frame
}

#[pyfunction]
pub fn build_response_frame(
    request_id: u16,
    slave_id: u8,
    data_value: u16,
) -> Vec<u8> {
    let mut frame = Vec::with_capacity(RESPONSE_FRAME_LENGTH);
    frame.push(START_BYTE);
    frame.push(RESPONSE_FRAME_TYPE);

    frame.push((request_id >> 8) as u8);
    frame.push((request_id & 0xFF) as u8);

    frame.push(slave_id);
    frame.push(0x02);

    frame.push((data_value >> 8) as u8);
    frame.push((data_value & 0xFF) as u8);

    let crc = crc16(&frame);
    frame.push((crc & 0xFF) as u8);
    frame.push((crc >> 8) as u8);

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

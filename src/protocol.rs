use pyo3::prelude::*;
use crate::crc::crc16;
use crate::util::{pack, unpack};

pub const START_BYTE_MASTER: u8 = 0xAA;
pub const START_BYTE_SLAVE: u8 = 0xBB;
pub const PACKET_LENGTH: usize = 10;


#[pyclass]
pub struct PacketConstants;

#[pymethods]
impl PacketConstants {
    #[classattr]
    pub const START_BYTE_MASTER: u8 = START_BYTE_MASTER;
    #[classattr]
    pub const START_BYTE_SLAVE: u8 = START_BYTE_SLAVE;
    #[classattr]
    pub const PACKET_LENGTH: usize = PACKET_LENGTH;
}


#[pyfunction]
pub fn build_request(
    slave_id: u8,
    uid: u8,
    cmd: u8,
    reg: u16,
    qty: u16,
) -> Vec<u8> {
    let mut packet = Vec::with_capacity(PACKET_LENGTH);
    packet.push(START_BYTE_MASTER);
    packet.push(slave_id);
    packet.push(uid);
    packet.push(cmd);
    packet.push((reg >> 8) as u8);
    packet.push((reg & 0xFF) as u8);
    packet.push((qty >> 8) as u8);
    packet.push((qty & 0xFF) as u8);

    let crc = crc16(&packet);
    packet.push((crc & 0xFF) as u8);
    packet.push((crc >> 8) as u8);

    packet
}

#[pyfunction]
pub fn build_ping_request(slave_id: u8, uid: u8) -> Vec<u8> {
    build_request(slave_id, uid, 0x00, 0, 0)
}

#[pyfunction]
pub fn build_register_request(
    slave_id: u8,
    uid: u8,
    internal_id: u8,
    pin: u8,
    type_code: u8,
    mode_code: u8,
) -> Vec<u8> {
    let reg = pack(internal_id, pin);
    let qty = pack(type_code, mode_code);
    build_request(slave_id, uid, 0x01, reg, qty)
}

#[pyfunction]
pub fn build_read_request(
    slave_id: u8,
    uid: u8,
    internal_id: u8,
) -> Vec<u8> {
    let reg = pack(0, internal_id); // or pack(internal_id, 0)
    build_request(slave_id, uid, 0x02, reg, 0)
}

#[pyfunction]
pub fn build_set_request(
    slave_id: u8,
    uid: u8,
    internal_id: u8,
    state: u8,
) -> Vec<u8> {
    let reg = pack(0, internal_id);
    let qty = pack(0, state); // state in low byte
    build_request(slave_id, uid, 0x03, reg, qty)
}

#[pyfunction]
pub fn check_crc(response: Vec<u8>) -> bool {
    if response.len() < 2 {
        return false;
    }

    let payload = &response[..response.len() - 2];
    let crc_calc = crc16(payload);

    let crc_lo = response[response.len() - 2];
    let crc_hi = response[response.len() - 1];

    let calc_lo = (crc_calc & 0xFF) as u8;
    let calc_hi = (crc_calc >> 8) as u8;

    crc_lo == calc_lo && crc_hi == calc_hi
}

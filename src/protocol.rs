use pyo3::prelude::*;
use crate::crc::crc16;
use crate::util::{pack, unpack};

pub const PACKET_LENGTH: usize = 10;

#[pyfunction]
pub fn build_request(
    slave_id: u8,
    uid: u8,
    cmd: u8,
    reg: u16,
    qty: u16,
) -> Vec<u8> {
    let mut packet = Vec::with_capacity(PACKET_LENGTH);
    packet.push(0xAA);
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

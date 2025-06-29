use pyo3::prelude::*;
use crate::crc::crc16;

#[pyfunction]
pub fn build_request(
    slave_id: u8,
    uid:      u8,
    cmd:      u8,
    reg:      u16,
    qty:      u16,
) -> Vec<u8> {
    let mut packet = Vec::with_capacity(10);
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

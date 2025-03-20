use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use rosc::{OscMessage, OscPacket, OscType, encoder};

pub trait Sendable {
    fn send(
        &self,
        socket: &UdpSocket,
        osc_addr: &str,
        dst_addr: &SocketAddrV4,
    ) -> Result<(), Box<dyn Error>>;
}
impl Sendable for f32 {
    fn send(
        &self,
        socket: &UdpSocket,
        osc_addr: &str,
        dst_addr: &SocketAddrV4,
    ) -> Result<(), Box<dyn Error>> {
        let message = OscPacket::Message(OscMessage {
            addr: osc_addr.to_string(),
            args: vec![OscType::Float(*self)],
        });
        let buffer = encoder::encode(&message)?;
        socket.send_to(&buffer, dst_addr)?;
        Ok(())
    }
}

pub struct OscSender {
    socket: UdpSocket,
    dst_addr: SocketAddrV4,
}
impl OscSender {
    pub fn new(
        src_address: Ipv4Addr,
        src_port: u16,
        dst_address: Ipv4Addr,
        dst_port: u16,
    ) -> OscSender {
        let socket = UdpSocket::bind(SocketAddrV4::new(src_address, src_port))
            .expect("couldn't bind to address");
        let dst_addr = SocketAddrV4::new(dst_address, dst_port);
        OscSender { socket, dst_addr }
    }
    pub fn send<T: Sendable>(&self, data: &T, osc_addr: &str) -> Result<(), Box<dyn Error>> {
        data.send(&self.socket, osc_addr, &self.dst_addr)
    }
}

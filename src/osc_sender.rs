use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use rosc::{OscMessage, OscPacket, OscType, encoder};


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
    pub fn send(&self, osc_packet: &OscPacket) -> Result<(), Box<dyn Error>> {
        let buffer = encoder::encode(osc_packet)?;
        self.socket.send_to(&buffer, self.dst_addr)?;
        Ok(())
    }
}

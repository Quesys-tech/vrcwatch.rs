use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use tokio::net::UdpSocket;

use rosc::{encoder, OscMessage, OscPacket, OscType};

/// Verify that an OSC address is valid.
///
/// # Errors
///
/// This function will return an error if is OSC address is invalid.
/// see https://opensoundcontrol.stanford.edu/spec-1_0.html
fn verify_osc_addr(addr: &str) -> Result<(), Box<dyn Error>> {
    if !addr.starts_with('/') {
        return Err("OSC address must start with '/'".into());
    }
    let forbidden_chars = [' ', '#', ',', '?', '[', ']', '{', '}'];
    for c in forbidden_chars {
        if addr.contains(c) {
            return Err(format!("OSC address cannot contain '{}'", c).into());
        }
    }
    Ok(())
}

pub trait Sendable {
    async fn send(
        &self,
        socket: &UdpSocket,
        osc_addr: &str,
        dst_addr: &SocketAddrV4,
    ) -> Result<(), Box<dyn Error>>;
}
impl Sendable for f32 {
    async fn send(
        &self,
        socket: &UdpSocket,
        osc_addr: &str,
        dst_addr: &SocketAddrV4,
    ) -> Result<(), Box<dyn Error>> {
        let message = OscPacket::Message(OscMessage {
            addr: osc_addr.to_string(),
            args: vec![OscType::Float(*self)],
        });
        verify_osc_addr(osc_addr)?;
        let buffer = encoder::encode(&message)?;
        socket.send_to(&buffer, dst_addr).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OscSender {
    socket: Arc<UdpSocket>,
    dst_addr: SocketAddrV4,
}
impl OscSender {
    pub async fn new(
        src_address: Ipv4Addr,
        src_port: u16,
        dst_address: Ipv4Addr,
        dst_port: u16,
    ) -> OscSender {
        let socket = UdpSocket::bind(SocketAddrV4::new(src_address, src_port))
            .await
            .expect("couldn't bind to address");
        let dst_addr = SocketAddrV4::new(dst_address, dst_port);
        OscSender {
            socket: Arc::new(socket),
            dst_addr,
        }
    }
    pub async fn send<T: Sendable>(&self, data: &T, osc_addr: &str) -> Result<(), Box<dyn Error>> {
        data.send(&self.socket, osc_addr, &self.dst_addr).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_osc_addr() {
        assert!((verify_osc_addr("/a/b/c/d/e")).is_ok());
        assert!((verify_osc_addr("a/b/c/d/e")).is_err());
        assert!((verify_osc_addr("/a b/c/d/e")).is_err());
        assert!((verify_osc_addr("/a/b#/c/d/e")).is_err());
        assert!((verify_osc_addr("/a/b/?c/d/e")).is_err());
        assert!((verify_osc_addr("/a/b/c/[d/e")).is_err());
        assert!((verify_osc_addr("/a/b/c/d/e]")).is_err());
        assert!((verify_osc_addr("/a/b/c/?d/e")).is_err());
        assert!((verify_osc_addr("/a/b/c/d/e?")).is_err());
    }
}

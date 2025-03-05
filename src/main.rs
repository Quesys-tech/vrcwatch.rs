use chrono::{Local, Timelike};
use std::error::Error;
use std::net::Ipv4Addr;
use std::net::{SocketAddrV4, UdpSocket};
use std::thread;
use std::time::Duration;

use clap::Parser;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: Ipv4Addr,
    #[arg(short, long, default_value = "9000")]
    port: u16,
    #[arg(short, long)]
    verbose: bool,
}

struct OscSender {
    socket: UdpSocket,
    dst_addr: SocketAddrV4,
}
impl OscSender {
    fn new(
        src_address: Ipv4Addr,
        src_port: u16,
        dst_address: Ipv4Addr,
        dst_port: u16,
    ) -> OscSender {
        let socket = UdpSocket::bind(format!("{}:{}", src_address, src_port))
            .expect("couldn't bind to address");
        let dst_addr = SocketAddrV4::new(dst_address, dst_port);
        OscSender { socket, dst_addr }
    }
    fn send(&self, osc_packet: &OscPacket) -> Result<(), Box<dyn Error>> {
        let buffer = encoder::encode(osc_packet)?;
        self.socket.send_to(&buffer, self.dst_addr)?;
        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
    }
    println!("Destination port: {}:{}", cli.address, cli.port);

    let sender = OscSender::new(Ipv4Addr::new(127, 0, 0, 1), 34254, cli.address, cli.port);

    loop {
        let x = 0.1;
        let y = 0.2;

        let message = OscPacket::Message(OscMessage {
            addr: "/3/xy1".to_string(),
            args: vec![OscType::Float(x), OscType::Float(y)],
        });
        sender.send(&message).expect("Error sending OSC message");

        let now = Local::now();
        println!("{}", now);

        // let msec_f = now.nanosecond() as f64 / 1_000_000_000.0;
        let second_fa = (now.second() as f64) / 60.0;
        let minute_fa = (now.minute() as f64 + second_fa) / 60.0;
        let hour_fa = (now.hour() as f64 +minute_fa) / 24.0;

        let hour_f_msg = OscPacket::Message(OscMessage {
            addr: "/avatar/parameters/DateTimeHourFA".to_string(),
            args: vec![OscType::Float(hour_fa as f32)],
        });
        sender.send(&hour_f_msg).expect("Error sending OSC message");
        let minute_f_msg = OscPacket::Message(OscMessage {
            addr: "/avatar/parameters/DateTimeMinuteFA".to_string(),
            args: vec![OscType::Float(minute_fa as f32)],
        });
        sender
            .send(&minute_f_msg)
            .expect("Error sending OSC message");
        let second_f_msg = OscPacket::Message(OscMessage {
            addr: "/avatar/parameters/DateTimeSecondFA".to_string(),
            args: vec![OscType::Float(second_fa as f32)],
        });
        sender
            .send(&second_f_msg)
            .expect("Error sending OSC message");

        thread::sleep(Duration::from_millis(200));
    }
}

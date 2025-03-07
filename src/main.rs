use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use chrono::{Local, TimeZone, Timelike};
use clap::Parser;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use tokio::signal;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1", help = "destination IP address")]
    address: Ipv4Addr,
    #[arg(short, long, default_value = "9000", help = "destination port")]
    port: u16,
    #[arg(short, long, help = "enable verbose mode")]
    verbose: bool,
    #[arg(short, long, help = "demo mode, the watch shows 10:08:42")]
    demo: bool,
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

async fn tick_watch(cli: &Cli, sender: &OscSender) -> Result<(), Box<dyn Error>> {
    let now = Local::now();

    let second_fa = (now.second() as f64) / 60.0;
    let minute_fa = (now.minute() as f64 + second_fa) / 60.0;
    let hour_fa = (now.hour() as f64 + minute_fa) / 24.0;

    let second_f_msg = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeSecondFA".to_string(),
        args: vec![OscType::Float(second_fa as f32)],
    });
    let minute_f_msg = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeMinuteFA".to_string(),
        args: vec![OscType::Float(minute_fa as f32)],
    });
    let hour_f_msg = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeHourFA".to_string(),
        args: vec![OscType::Float(hour_fa as f32)],
    });

    sender.send(&second_f_msg)?;
    sender.send(&minute_f_msg)?;
    sender.send(&hour_f_msg)?;
    if cli.verbose {
        println!("{}:{}:{}", now.hour(), now.minute(), now.second());
    }
    Ok(())
}

async fn update_second_change(cli: Cli, sender: OscSender) {
    loop {
        let now = Local::now();
        let sub_second = now.timestamp_subsec_nanos();
        let sleep_duration = Duration::from_nanos(1_000_000_000 - sub_second as u64);
        if cli.verbose {
            println!("Sleeping for {}ms", sleep_duration.as_millis());
        }
        sleep(sleep_duration).await;
        tick_watch(&cli, &sender).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
    }
    println!("Destination port: {}:{}", cli.address, cli.port);

    let sender = OscSender::new(Ipv4Addr::new(127, 0, 0, 1), 34254, cli.address, cli.port);
    match cli.demo {
        true => {
            tokio::spawn(demo_mode(sender));
        }
        false => {
            tokio::spawn(update_second_change(cli, sender));
        }
    }
    println!("Press Ctrl-C to exit");
    let mut sigint_handler = signal::windows::ctrl_c().unwrap();
    sigint_handler.recv().await;
    println!("Exiting...");
}

async fn demo_mode(sender: OscSender) {
    let display_time = Local.with_ymd_and_hms(2017, 2, 1, 10, 8, 42).unwrap(); // https://museum.seiko.co.jp/knowledge/trivia01/

    println!("Display mode: fixed at {}", display_time);

    let second_fa = (display_time.second() as f64) / 60.0;
    let minute_fa = (display_time.minute() as f64 + second_fa) / 60.0;
    let hour_fa = (display_time.hour() as f64 + minute_fa) / 24.0;

    let second_f_msg = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeSecondFA".to_string(),
        args: vec![OscType::Float(second_fa as f32)],
    });
    let minute_f_msg = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeMinuteFA".to_string(),
        args: vec![OscType::Float(minute_fa as f32)],
    });
    let hour_f_msg = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeHourFA".to_string(),
        args: vec![OscType::Float(hour_fa as f32)],
    });

    loop {
        sender.send(&second_f_msg).unwrap();
        sender.send(&minute_f_msg).unwrap();
        sender.send(&hour_f_msg).unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}

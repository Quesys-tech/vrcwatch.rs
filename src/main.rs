use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::SystemTime;

use chrono::{DateTime, Local, TimeZone, Timelike, Utc};
use clap::Parser;
use moon_phase::MoonPhase;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use tokio::signal;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "127.0.0.1",
        help = "destination IP address"
    )]
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
        let socket = UdpSocket::bind(SocketAddrV4::new(src_address, src_port))
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

async fn send_time(
    sender: &OscSender,
    time: &DateTime<Local>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let second_fraction = (time.second() as f64) / 60.0;
    let minute_fraction = (time.minute() as f64 + second_fraction) / 60.0;
    let hour_fraction = (time.hour() as f64 + minute_fraction) / 24.0;

    let second_animation = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeSecondFA".to_string(),
        args: vec![OscType::Float(second_fraction as f32)],
    });
    let minute_animation = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeMinuteFA".to_string(),
        args: vec![OscType::Float(minute_fraction as f32)],
    });
    let hour_animation = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/DateTimeHourFA".to_string(),
        args: vec![OscType::Float(hour_fraction as f32)],
    });

    sender.send(&second_animation)?;
    sender.send(&minute_animation)?;
    sender.send(&hour_animation)?;

    if verbose {
        println!("{}:{}:{}", time.hour(), time.minute(), time.second());
    }
    Ok(())
}

/// Send moon phase to the watch (address: /avatar/parameters/MoonPhaseF, type: float)
async fn send_moon_phase(sender: &OscSender, moon_phase: f32) -> Result<(), Box<dyn Error>> {
    let moon_phase_animation = OscPacket::Message(OscMessage {
        addr: "/avatar/parameters/MoonphaseF".to_string(),
        args: vec![OscType::Float(moon_phase)],
    });
    sender.send(&moon_phase_animation)?;
    Ok(())
}

/// Local time to moon phase (0.0: new moon, 0.5: full moon, 1.0: new moon)
async fn calc_moon_phase<Tz: TimeZone>(local_time: &DateTime<Tz>) -> f32 {
    let system_time: SystemTime = local_time.with_timezone(&Utc).into();
    let moon_phase = MoonPhase::new(system_time);

    moon_phase.phase as f32
}
#[tokio::test]
async fn test_calc_moon_phase() {
    let full_moon_list = [
        (2025, 1, 13, 22, 27),
        (2025, 2, 12, 13, 53),
        (2025, 3, 14, 6, 55),
        (2025, 4, 13, 0, 22),
        (2025, 5, 12, 16, 56),
        (2025, 6, 11, 07, 44),
    ];

    for (year, month, day, hour, min) in full_moon_list {
        let local_time = Utc
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .unwrap();
        let moon_phase = calc_moon_phase(&local_time).await;
        let error = moon_phase - 0.5;
        assert!(
            error.abs() < 0.01 / 0.5, // 1% error
            "full moon at {}: calc:{}",
            local_time,
            moon_phase
        );
    }
}

async fn tick_watch(cli: &Cli, sender: &OscSender) -> Result<(), Box<dyn Error>> {
    let now = Local::now();
    send_time(sender, &now, cli.verbose).await?;
    let moon_phase = calc_moon_phase(&now).await;
    send_moon_phase(sender, moon_phase).await?;

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

    let sender = OscSender::new(Ipv4Addr::new(127, 0, 0, 1), 0, cli.address, cli.port);
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

    loop {
        send_time(&sender, &display_time, true).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}

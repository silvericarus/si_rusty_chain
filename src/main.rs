use std::env;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::process;

struct Args {
    host: String,
    port: u16,
    mode: String,
    address: SocketAddr,
}

fn check_ip(config: &mut Args) -> Result<(), Box<dyn std::error::Error>> {
    let ip = config
        .host
        .parse::<IpAddr>()
        .expect("Error parsing address");
    let collect: Vec<SocketAddr> = (ip, config.port).to_socket_addrs()?.collect();
    let chosen: SocketAddr = collect.first().cloned().expect("No address collected");
    config.address = chosen;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 && args.len() != 2 {
        eprintln!("Not enough arguments provided.");
        eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        process::exit(1);
    } else if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        process::exit(0);
    } else if args.len() == 7 {
        let mut config = Args {
            host: String::from(&args[2]),
            port: args[4].parse().expect("Error parsing."),
            mode: String::from(&args[6]),
            address: SocketAddr::new([127, 0, 0, 1].into(), 80),
        };

        if config.port <= 1024 || args[3] != "--port" {
            eprintln!("Unvalid arguments provided.");
            eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
            process::exit(1);
        }
        if config.mode != "demo" {
            eprintln!("Unvalid mode provided. Currently only the 'demo' mode is available.");
            eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
            process::exit(1);
        }
        let _ = check_ip(&mut config);
        println!("Host: {}", config.address);
        println!("Mode: {}", config.mode);
        process::exit(0);
    } else {
        eprintln!("Unvalid arguments provided.");
        eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        process::exit(1);
    }
}

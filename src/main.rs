use std::env;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::process;

struct Args {
    host: String,
    port: u16,
    mode: String,
    final_host: SocketAddr,
}

fn check_ip(config: &mut Args) -> Result<(), Box<dyn std::error::Error>> {
    let ip = config.host.parse::<IpAddr>().ok();
    if ip.is_some() {
        let collect: Vec<SocketAddr> = (ip.ok_or("Invalid Address")?, config.port)
            .to_socket_addrs()?
            .collect();
        let addr = collect
            .iter()
            .find(|a| a.is_ipv4())
            .copied()
            .unwrap_or(collect[0]);
        config.final_host = addr;
    } else {
        match (config.host.as_str(), config.port).to_socket_addrs() {
            Ok(it) => {
                let collect: Vec<SocketAddr> = it.collect();

                let addr = collect
                    .iter()
                    .find(|a| a.is_ipv4())
                    .copied()
                    .unwrap_or(collect[0]);
                config.final_host = addr;
            }
            Err(_e) => {
                eprintln!("Error. Unknown host.");
                process::exit(1);
            }
        }
    }

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
            final_host: SocketAddr::new([127, 0, 0, 1].into(), 80),
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
        println!("Host: {}", config.final_host);
        println!("Mode: {}", config.mode);
        process::exit(0);
    } else {
        eprintln!("Unvalid arguments provided.");
        eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        process::exit(1);
    }
}

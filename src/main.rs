mod wal;
use flexi_logger::Logger;
use log::{debug, error, info, warn};
use core::error;
use std::env;
use std::error::Error;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::process;
use std::time::Instant;

use crate::wal::{ensure_wal_dir, open_init_current};

struct Args {
    host: String,
    port: u16,
    mode: String,
    final_host: SocketAddr,
}

fn init_logging() -> Result<(), Box<dyn Error>> {
    let lvl = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    match Logger::try_with_str(&lvl) {
        Ok(builder) => {
            builder.format(flexi_logger::opt_format).start()?;
        }
        Err(_) => {
            eprintln!("LOG_LEVEL='{}' inválido, usando 'info'", lvl);
            Logger::try_with_str("info")?
                .format(flexi_logger::opt_format)
                .start()?;
        }
    }
    Ok(())
}

fn check_ip(config: &mut Args) -> Result<(), Box<dyn std::error::Error>> {
    let r0 = Instant::now();
    let resolve_ms = r0.elapsed().as_millis();
    let ip = config.host.parse::<IpAddr>().ok();
    let kind = if ip.is_some() { "literal" } else { "dns" };
    if ip.is_some() {
        let collect: Vec<SocketAddr> = (ip.ok_or("Invalid Address")?, config.port)
            .to_socket_addrs()?
            .collect();
        debug!(
            "dns stats dns_total={} v4={} v6={} resolve_ms={}ms",
            collect.len(),
            collect.iter().filter(|a| a.is_ipv4()).count(),
            collect.iter().filter(|a| a.is_ipv6()).count(),
            resolve_ms
        );
        let addr = collect
            .iter()
            .find(|a| a.is_ipv4())
            .copied()
            .unwrap_or(collect[0]);
        info!(
            "resolved addr={} kind={} is_ipv4={}",
            addr,
            kind,
            addr.is_ipv4()
        );
        config.final_host = addr;
    } else {
        match (config.host.as_str(), config.port).to_socket_addrs() {
            Ok(it) => {
                let collect: Vec<SocketAddr> = it.collect();
                debug!(
                    "dns stats dns_total={} v4={} v6={} resolve_ms={}ms",
                    collect.len(),
                    collect.iter().filter(|a| a.is_ipv4()).count(),
                    collect.iter().filter(|a| a.is_ipv6()).count(),
                    resolve_ms
                );
                let addr = collect
                    .iter()
                    .find(|a| a.is_ipv4())
                    .copied()
                    .unwrap_or(collect[0]);
                info!(
                    "resolved addr={} kind={} is_ipv4={}",
                    addr,
                    kind,
                    addr.is_ipv4()
                );
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

    if args.len() < 7 && args.len() != 2 {
        eprintln!("Not enough arguments provided.");
        eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        process::exit(1);
    } else if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        process::exit(0);
    } else if args.len() == 7 || args.len() == 8 {
        let mut config = Args {
            host: String::from(&args[2]),
            port: args[4].parse().expect("Error parsing."),
            mode: String::from(&args[6]),
            final_host: SocketAddr::new([127, 0, 0, 1].into(), 80),
        };
        let rotate_now: bool = std::env::args().any(|a| a == "--rotate-now");

        let _ = init_logging();
        let t0: Instant = Instant::now();
        info!(
            "start app=si_rusty_chain version={} mode={} host={} port={}",
            env!("CARGO_PKG_VERSION"),
            config.mode,
            config.host,
            config.port
        );
        let wal_dir: &Path = Path::new("data/wal");
		let _ = ensure_wal_dir(wal_dir).map_err(|e| {
			 error!("wal_open: dir couldn't be created {:?}: {}", wal_dir, e);
    	});

		let current_path: std::path::PathBuf = wal_dir.join("current.wal");
		let mut wal_file = open_init_current(&current_path).map_err(|e| {
			error!("wal_open: failed to open {:?}: {}", current_path, e);
		});

		//TODO: Cambiar a unwrap seguro linea 133 y 138
		let cur_size = wal_file.unwrap().metadata()?.len();
		info!("wal_open path={:?} size={}B", current_path, cur_size);

        warn!("'demo' mode is only a testing temporary solution. Expect it to change.");

        if config.port <= 1024 || args[3] != "--port" {
            eprintln!("Unvalid arguments provided.");
            eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
            error!("Unvalid port provided, exiting.");
            process::exit(1);
        }
        if config.mode != "demo" {
            eprintln!("Unvalid mode provided. Currently only the 'demo' mode is available.");
            eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
            error!("Unvalid mode provided, exiting.");
            process::exit(1);
        }
        let _ = check_ip(&mut config);
        let startup_ms = t0.elapsed().as_millis();
        info!("shutdown startup_ms={}ms", startup_ms);
        process::exit(0);
    } else {
        eprintln!("Unvalid arguments provided.");
        eprintln!("./si_rusty_chain --host <host> --port <port> --mode <mode>");
        error!("Unvalid arguments provided, exiting.");
        process::exit(1);
    }
}

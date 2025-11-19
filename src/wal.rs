use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use log::{error, info};

pub const MAGIC: &[u8] = b"SIRWAL\0";
pub const VERSION: u16 = 1;

pub fn ensure_wal_dir(dir: &Path) -> io::Result<()> {
    fs::create_dir_all(dir)
}

pub fn open_init_current(path: &Path) -> io::Result<File> {
    let mut f: File = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)?;

    let mut size: u64 = f.metadata()?.len();
    let header_len: u64 = (MAGIC.len() + 2) as u64;

    if size == 0 {
        f.write_all(super::wal::MAGIC)?;
        f.write_all(&super::wal::VERSION.to_le_bytes())?;
        f.sync_all()?;
    }

    size = f.metadata()?.len();

    if size < header_len {
        error!("wal header truncated: size={}B (< {}B)", size, header_len);
        std::process::exit(1);
    }

    f.seek(SeekFrom::Start(0))?;

    let mut magic: [u8; 7] = [0u8; 7];
    f.read_exact(&mut magic)?;
    if magic != MAGIC {
        error!("magic mismatch: expected {:?}, got {:?}", MAGIC, magic);
        std::process::exit(1);
    }

    let mut ver: [u8; 2] = [0u8; 2];
    f.read_exact(&mut ver)?;
    let v: u16 = u16::from_le_bytes(ver);
    if v != VERSION {
        error!("version mismatch: expected {:?}, got {:?}", VERSION, v);
        std::process::exit(1);
    }

    Ok(f)
}

pub fn rotate_wal(dir: &Path) -> io::Result<(PathBuf, PathBuf)> {
    let t0: SystemTime = SystemTime::now();
    let current: PathBuf = dir.join("current.wal");
    let size_old: u64 = fs::metadata(&current)?.len();

    let ts: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let rotated: PathBuf = dir.join(format!("wal_{}.wal", ts));

    match fs::rename(&current, &rotated) {
        Ok(_ok) => {
            info!("wal_rename file renamed successfully")
        }
        Err(e) => {
            error!("wal_rename error renaming the file: {}", e)
        }
    };

    let mut f_new: File = File::create(&current)?;
    f_new.write_all(super::wal::MAGIC)?;
    f_new.write_all(&super::wal::VERSION.to_le_bytes())?;
    f_new.sync_all()?;

    let elapsed_ms = SystemTime::elapsed(&t0);
    let elapsed_duration = match elapsed_ms {
        Ok(ms) => ms,
        Err(_e) => {
            std::process::exit(1);
        }
    };
    info!(
        "wal_rotate from={} to={} size_old={}B elapsed_ms={}ms",
        current.display(),
        rotated.display(),
        size_old,
        elapsed_duration.as_millis()
    );
    Ok((current, rotated))
}

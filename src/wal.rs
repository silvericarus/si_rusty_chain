use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
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

    let size: u64 = f.metadata()?.len();
    if size == 0 {
        f.write_all(super::wal::MAGIC)?;
        f.write_all(&super::wal::VERSION.to_le_bytes())?;
        f.sync_all()?;
    }
    Ok(f)
}

pub fn rotate_wal(dir: &Path) -> io::Result<(PathBuf, PathBuf)> {
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

    info!("wal_rotate size of old wal file {}B", size_old);
    Ok((current, rotated))
}

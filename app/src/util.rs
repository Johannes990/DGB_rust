use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_seed() -> [u64; 2] {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time error");

    let nanos = since_epoch.as_nanos();
    [(nanos & 0xFFFFFFFFFFFFFFFF) as u64, (nanos >> 64) as u64]
}

use std::sync::Mutex;
use std::time::Instant;

pub static PREVIOUS: Mutex<Option<Instant>> = Mutex::new(None);
pub static INITIAL: Mutex<Option<Instant>> = Mutex::new(None);

pub fn timing(msg: &str) {
    let now = Instant::now();
    INITIAL.lock().unwrap().get_or_insert(now);
    let mut prev = PREVIOUS.lock().unwrap();
    if let Some(prev) = prev.replace(now) {
        let duration = now - prev;
        info!("{:20} {}ms", format!("{msg}:"), duration.as_millis());
    }
}

pub fn timing_done() {
    let now = Instant::now();
    let initial = INITIAL.lock().unwrap().unwrap();
    let duration = now - initial;
    info!("{:20} {}ms", "total:".to_string(), duration.as_millis());
}

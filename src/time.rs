use std::thread::sleep as thread_sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn ustime() -> u128 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    since_the_epoch.as_micros()
}

pub fn mstime() -> u128 {
    ustime() / 1000
}

pub fn sleep(ms: u64) {
    thread_sleep(Duration::from_millis(ms));
}

pub fn sleep_us(us: u64) {
    thread_sleep(Duration::from_micros(us));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ustime() {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let micros = since_the_epoch.as_micros();

        assert_eq!(ustime(), micros);
    }

    #[test]
    fn test_mstime() {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let micros = since_the_epoch.as_micros();
        let millis = micros / 1000;

        assert_eq!(mstime(), millis);
    }

    #[test]
    fn test_sleep() {
        let start = ustime();
        sleep(1000);
        let end = ustime();

        assert!(end - start >= 1000);
    }

    #[test]
    fn test_sleep_us() {
        let start = ustime();
        sleep_us(1000);
        let end = ustime();

        assert!(end - start >= 1000);
    }
}

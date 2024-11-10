use rand::seq::SliceRandom;

use crate::port;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ADDRESS: &str = "127.0.0.1";
pub const MAX_MESSAGE_SIZE: usize = 1024;

/// Spawns a new port number by randomly selecting through a range of port numbers
pub fn spawn_port() -> u16 {
    const PORT_LIST: [u16; 10] = [
        49152, 49153, 49154, 49155, 49156, 49157, 49158, 49159, 49160, 49161,
    ];

    let mut rng = rand::thread_rng();
    let port = PORT_LIST.choose(&mut rng).unwrap();

    drop(rng); // Drop the rng from memory

    let _ports = port::Port::new(*port);

    if let Ok(true) = _ports.is_closed() {
        // If the port is closed, return the another port
        return spawn_port();
    } else {
        return *port;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_port() {
        let port = spawn_port();
        assert!(port >= 49152 && port <= 49161); // Assert that the port is within the range
    }
}
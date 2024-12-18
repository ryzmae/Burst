use std::io;
use std::net::{SocketAddr, TcpListener};

pub struct Port {
    port: u16,
}

impl Port {
    pub fn new(port: u16) -> Self {
        Port { port }
    }

    pub fn is_open(&self) -> Result<bool, io::Error> {
        let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listerner: TcpListener = TcpListener::bind(addr)?;
        drop(listerner);
        Ok(true)
    }

    pub fn is_closed(&self) -> Result<bool, io::Error> {
        match self.is_open() {
            Ok(_) => Ok(false),
            Err(_) => Ok(true),
        }
    }

    pub fn is_valid(&self, ports: &[u16]) {
        for _port in ports {
            match self.is_open() {
                Ok(true) => (),
                Ok(false) => (),
                Err(_) => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_is_open() {
        let port = Port::new(5334);

        // if the port is already in use return true
        if port.is_open().is_err() {
            assert!(true);
        }
        assert_eq!(port.is_open().unwrap(), true);
    }

    #[test]
    fn test_port_is_closed() {
        let port = Port::new(5334);
        if port.is_closed().is_err() {
            assert!(false);
        }

        assert_eq!(port.is_closed().unwrap(), false);
    }

    #[test]
    fn test_port_is_valid() {
        let port = Port::new(5334);
        let ports = [3443, 8384, 3234];
        port.is_valid(&ports);
    }
}

use ::std::net::{SocketAddr, IpAddr, UdpSocket};
use ::std::time::{Instant, Duration};
use ::std::sync::mpsc;
use ::std::thread;
use ::std::cmp::Ordering;
use ::std::collections::BTreeSet;
use net2::UdpBuilder;
use net2::unix::UnixUdpBuilderExt;
use regex::Regex;

use errors::*;

const BROADCAST_ADDR: &'static str = "255.255.255.255:48899";
const BIND_ADDR: &'static str = "0.0.0.0:48900";
const COMMAND_PORT: u16 = 8899;
const QUERY_STRING: &'static str = "Link_Wi-Fi";
const QUERY_REPEAT: usize = 10;
const QUERY_DELAY: u32 = 50000;

#[derive(Debug, Eq, Clone)]
pub struct BridgeAddress {
    ip: IpAddr,
    id: String,
}

impl Ord for BridgeAddress {
    fn cmp(&self, other: &BridgeAddress) -> Ordering {
        self.ip.cmp(&other.ip)
    }
}

impl PartialOrd for BridgeAddress {
    fn partial_cmp(&self, other: &BridgeAddress) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for BridgeAddress {
    fn eq(&self, other: &BridgeAddress) -> bool {
        self.ip == other.ip
    }
}

pub struct Communicator {
    _private: (),
    socket: UdpSocket,
    parser: Regex,
}

impl Communicator {
    pub fn new() -> Result<Communicator> {
        let address = try!(BIND_ADDR.parse::<SocketAddr>());
        let parser = try!(Regex::new(r"(?P<ip>\d+\.\d+\.\d+\.\d+),(?P<macaddr>[0-9A-F]{12}),"));
        let builder = try!(UdpBuilder::new_v4());
        try!(builder.reuse_address(true));
        try!(builder.reuse_port(true));
        let socket = try!(builder.bind(address));
        try!(socket.set_broadcast(true));
        try!(socket.set_nonblocking(true));
        Ok(Communicator { _private: (), socket: socket, parser: parser })
    }

    pub fn send<T>(&self, cmd: T, address: &BridgeAddress) -> Result<()> where T: Into<Vec<u8>> {
        let bytes: Vec<u8> = cmd.into();
        let remote = SocketAddr::new(address.ip, COMMAND_PORT);
        try!(self.socket.send_to(bytes.as_slice(), remote));
        Ok(())
    }

    pub fn query(&self, timeout: Duration) -> Result<BTreeSet<BridgeAddress>> {
        let address = try!(BROADCAST_ADDR.parse::<SocketAddr>());
        let cmd = QUERY_STRING.as_bytes();
        let (tx, rx) = mpsc::channel();
        let listener = try!(self.socket.try_clone());
        let mut results = BTreeSet::new();

        let handle = thread::spawn(move || {
            let expiry = Instant::now() + timeout;
            let mut buf = [0u8; 64];
            loop {
                match listener.recv_from(&mut buf) {
                    Ok((byte_count, _)) => {
                        let data = Vec::from(&buf[0..byte_count]);
                        tx.send(Some(data)).unwrap();
                    },
                    Err(e) => {
                        match e.kind() {
                            ::std::io::ErrorKind::WouldBlock => {
                            },
                            _ => panic!(e)
                        }
                    }
                }
                if Instant::now() > expiry {
                    break;
                }
                thread::sleep(Duration::new(0, QUERY_DELAY));
            }
            tx.send(None).unwrap();
        });
        for _ in 0..QUERY_REPEAT {
            try!(self.socket.send_to(cmd, address));
            thread::sleep(Duration::new(0, QUERY_DELAY));
        }
        if handle.join().is_err() {
            return Err("Couldn't join listener thread".into());
        }
        loop {
            if let Some(value) = try!(rx.recv()) {
                let response = String::from_utf8_lossy(&value);
                let bridge = try!(self.parse_bridge_response(&response));
                results.insert(bridge);
            } else {
                break;
            }
        }
        Ok(results)
    }

    fn parse_bridge_response(&self, response: &str) -> Result<BridgeAddress> {
        if let Some(caps) = self.parser.captures(response) {
            if let Some(ip_string) = caps.name("ip") {
                if let Some(mac_address) = caps.name("macaddr") {
                    let ip = try!(ip_string.parse::<IpAddr>());
                    return Ok(BridgeAddress { ip: ip, id: mac_address.to_owned() })
                }
            }
        }
        Err("Response was not in a recognized format".into())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::time::Duration;

    #[test]
    fn test_query() {
        let co = Communicator::new().unwrap();
        co.query(Duration::from_secs(2)).unwrap();
    }

}

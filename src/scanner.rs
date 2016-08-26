use std::net::{Ipv4Addr, TcpStream, SocketAddrV4, Shutdown};
use std::str::FromStr;

const MIN_PORT: u16 = 0;
const MAX_PORT: u16 = 65535;
//const MaxPort: i32 = 65535;

pub fn parse_ipv4(ip: &str)  -> Result<Ipv4Addr, &'static str> {
    match Ipv4Addr::from_str(ip) {
        Ok(ipv4) => Ok(ipv4),
        _ => Err("Invalid ip address spec, must only have 4 octets")
    }
}

pub fn scan_ports(ip: Ipv4Addr) {
    for i in MIN_PORT..MAX_PORT {
        let stream = TcpStream::connect(SocketAddrV4::new(ip, i));
        if stream.is_ok() {
            println!("Port {} is open", i);
        }
    }
}

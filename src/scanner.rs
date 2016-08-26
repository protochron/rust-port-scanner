use std::net::{Ipv4Addr, TcpStream};
use std::str::FromStr;

pub fn parse_ipv4(ip: &str)  -> Result<Ipv4Addr, &'static str> {
    match Ipv4Addr::from_str(ip) {
        Ok(ipv4) => Ok(ipv4),
        _ => Err("Invalid ip address spec, must only have 4 octets")
    }
}

pub fn scan_ports(ip: &Ipv4Addr) {
}

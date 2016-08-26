extern crate getopts;
use getopts::Options;
use std::env;
use std::net::Ipv4Addr;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} ip_address", program);
    print!("{}", opts.usage(&brief));
}

fn parse_ipv4(ip: &str)  -> Result<Ipv4Addr, &'static str> {
   let split_ip: Vec<&str> = ip.split(".").collect();
   if split_ip.len() == 4 {
       //match Ipv4Addr::new(split_ip[0], split_ip[1], split_ip[2], split_ip[3]) {
       print!("works?");
   } else {
       Err("Invalid ip address spec");
   }
   Ok(Ipv4Addr::new(1,2,3,4));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return
    }

    let ip_address = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    print!("{}", ip_address);
}

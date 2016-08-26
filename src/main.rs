extern crate getopts;
use getopts::Options;
use std::env;
use std::process;

extern crate port_scanner;
use port_scanner::scanner::*;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} ip_address", program);
    print!("{}", opts.usage(&brief));
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
        process::exit(1)
    }

    let ip_address = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    match parse_ipv4(&ip_address) {
        Ok(ipv4) => {
            println!("Scanning {}", ipv4);
            scan_ports(ipv4)
        }
        Err(err) => {
            println!("{}", err);
        }
    };
}

extern crate getopts;
use getopts::Options;
use std::env;

extern crate port_scanner;

const MIN_PORT: i32 = 0;
const MAX_PORT: i32 = 1024;
//const MaxPort: i32 = 65535;

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
        return
    }

    let ip_address = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    match port_scanner::scanner::parse_ipv4(&ip_address) {
        Ok(x) => println!("{}", x),
        Err(err) => println!("{}", err),
    };
}

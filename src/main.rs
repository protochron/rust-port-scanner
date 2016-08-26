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

   match split_ip.len() {
       4 => {
           let converted_ip_octets = split_ip.iter().map(|x| x.parse::<u8>().unwrap())
               .collect::<Vec<u8>>();
           let parsed_ip = Ipv4Addr::new(converted_ip_octets[0],
                                         converted_ip_octets[1],
                                         converted_ip_octets[2],
                                         converted_ip_octets[3]);
           return Ok(parsed_ip);
       },
       _ => { return Err("Invalid ip address spec, must only have 4 octets") },
   }
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

    match parse_ipv4(&ip_address) {
        Ok(x) => println!("{}", x),
        Err(err) => println!("{}", err),
    };

}

extern crate clap;
extern crate redis;

use clap::{Arg, App};
use std::process;

fn main() {

    let cli_matches = App::new("check_redis_simple")
        .author("lagooj")
        .version("0.1")
        .about("Check redis status and memory usage, should be runned by icinga, nagios")
        .arg(Arg::with_name("WARNING")
             .short("w")
             .long("warning")
             .help("Set warning threshold")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("CRITICAL")
             .short("c")
             .long("critical")
             .help("Set critical threshold")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("HOSTNAME:PORT")
             .short("H")
             .long("hostname")
             .help("Set hostname and port ex: localhost:6379")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("v")
             .short("v")
             .help("Set verbose mode")
             .multiple(false))
        .get_matches();

    let warning_ts :u8 = cli_matches.value_of("WARNING").unwrap().parse::<u8>().unwrap();
    let critical_ts :u8 = cli_matches.value_of("CRITICAL").unwrap().parse::<u8>().unwrap();
    let hostname = cli_matches.value_of("HOSTNAME:PORT").unwrap();
    let percent = match get_memory_values(hostname) {
        Err(e) => panic!(e),
        Ok(v) => compute_percent(v),
    };

    match percent {
        _ if percent > critical_ts => println!("Critical triggered"),
        _ if percent > warning_ts => println!("Warning triggered"),
        _ => process::exit(0)
    };
    process::exit(0);
}

pub fn get_memory_values(hostname: &str) -> Result<Vec<usize>,redis::RedisError> {

    let redis_host= format!("redis://{}", hostname);
    let client =  redis::Client::open(&*redis_host).unwrap();
    let con = client.get_connection().unwrap();
    let info :redis::InfoDict = redis::cmd("INFO").query(&con).unwrap();
    let mut memory :Vec<usize> = Vec::new();

    memory.push( info.get("maxmemory").unwrap_or(0) );
    memory.push( info.get("used_memory").unwrap_or(0) );

    return Ok(memory);
}

fn compute_percent(mem: Vec<usize>) -> u8 {

    let max  = mem[0];
    let used = mem[1];
    println!("{} - {}", used as u32, max  as u32);
    println!("{}", ((used as f64 / max as f64) * 100.00));
    if max == 0  || used == 0 {
        return 100;
    }
    else {
        return ( (used as f64 / max as f64) * 100.00 ) as u8;
    }
}

extern crate clap;
extern crate redis;

use clap::{Arg, App, ArgMatches};
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

    let warning_ts :u8 = get_warn_tres(&cli_matches);
    let critical_ts :u8 = get_crit_tres(&cli_matches);
    let hostname = cli_matches.value_of("HOSTNAME:PORT").unwrap();

    let percent = match get_memory_values(hostname) {
        Err(e) => panic!(e),
        Ok(v) => compute_percent(&v, get_verbose(&cli_matches)),
    };

    match percent {
        _ if percent > critical_ts => {
            println!("Critical : {}% used", percent);
            process::exit(2)
        },
        _ if percent > warning_ts => {
            println!("Warning : {}% used", percent);
            process::exit(1)
        },
        _ => {
            println!("OK : {}% used", percent);
            process::exit(0)
        }
    };
}

pub fn get_warn_tres(matches :&ArgMatches) -> u8 {
    matches.value_of("WARNING").expect("Warning threshold must be set").parse::<u8>().expect("And must be a u8")
}

pub fn get_crit_tres(matches :&ArgMatches) -> u8 {
    matches.value_of("CRITICAL").expect("Critical threshold must be set").parse::<u8>().expect("And must be a u8")
}
pub fn get_verbose(matches :&ArgMatches) -> bool {
    matches.is_present("v")
}
pub fn get_memory_values(hostname: &str) -> Result<Vec<usize>,redis::RedisError> {

    let redis_host= format!("redis://{}", hostname);
    let mut memory :Vec<usize> = Vec::new();

    let client = match redis::Client::open(&*redis_host) {
        Ok(client) => client,
        Err(error) => {
            println!("UNKNOWN : {}", error );
            process::exit(3);
        }
    };
    let con = match client.get_connection() {
        Ok(con) => {
            con.set_read_timeout(Some(std::time::Duration::new(5,0))).expect("Can't set read timeout");
            con.set_write_timeout(Some(std::time::Duration::new(5,0))).expect("Can't set write timeout");
            con
        },
        Err(error) => {
            println!("UNKNOWN : {}", error );
            process::exit(3);
        }
    };
    let info :redis::InfoDict = match redis::cmd("INFO").query(&con) {
        Ok(info) => info,
        Err(error) => {
            println!("UNKNOWN : {}", error );
            process::exit(3);
        }
    };

    memory.push( info.get("maxmemory").unwrap_or(0) );
    memory.push( info.get("used_memory").unwrap_or(0) );

    Ok(memory)
}

fn compute_percent(mem: &[usize], verbose: bool) -> u8 {

    let max  = mem[0];
    let used = mem[1];
    if verbose {
        println!("{} - {}", used as u32, max  as u32);
        println!("{}", ((used as f64 / max as f64) * 100.00));
    }
    if max == 0  || used == 0 {
        100
    }
    else {
        ( (used as f64 / max as f64) * 100.00 ) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_50() {
        let vec = vec![24,12];
        assert_eq!(compute_percent( &vec, false),50);
    }
}

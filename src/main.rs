extern crate clap;
extern crate redis;


use clap::{Arg, App};

use redis::Commands;



fn main() {

    let cli_matches = App::new("check_redis_simple")
        .author("M3")
        .version("0.1")
        .about("Check redis status and memory usage, should be runned by icinga")
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
             .takes_value(true))
        .get_matches();

       let warning_ts = cli_matches.value_of("WARNING").unwrap();
       let critical_ts = cli_matches.value_of("CRITICAL").unwrap();
       let hostname = cli_matches.value_of("HOSTNAME:PORT").unwrap();

    test_redis(hostname, warning_ts, critical_ts);
}
pub fn test_redis(hostname: &str, warning: &str, critical: &str){
    // Open a connection
    let redis_host= format!("redis://{}", hostname);
    let client =  redis::Client::open(&*redis_host).unwrap();
    let con = client.get_connection().unwrap();
    let info :redis::InfoDict = redis::cmd("INFO").query(&con).unwrap();
    let max_memory :Option<usize> = info.get("max_memory");
    let used_memory :Option<usize> = info.get("used_memory");
    println!("{:?}",max_memory.unwrap_or(0));
    println!("{:?}",used_memory.unwrap_or(0));
    //println!("{:?}", client);
/*    let client = match client {
        Ok(o) => { println!("{:?}",o); o },
        Err(e) => println!("Erreur:{:?}", e),
    };*/

    // set key = “Hello World”
    let _: () = match client.set("key","Hello World") {
        Ok(o) => o,
        Err(e) => println!("{:?}", e),
    };

    // get key
    let key : String = client.get("key").unwrap();
       println!("Value of config: {}", warning);
       println!("Value of critical: {}", critical);
       println!("Value of hostname: {}", hostname);
    println!("key: {}", key);
    println!("{:?}", compute_percent(max_memory.unwrap_or(0),used_memory.unwrap_or(0) ));
}

fn compute_percent( max :usize, used :usize ) -> u32 {
    if max == 0  || used == 0 {
        return 100;
    }
    else {
        return ( (max / used) * 100 ) as u32;
    }
}

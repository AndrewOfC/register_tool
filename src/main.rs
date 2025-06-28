mod lib;

use clap::{Command, Arg};

fn main() {
    let matches = Command::new("register_tool")
        .version("0.1.0")
        .author("Register Tool Developer")
        .about("Memory register read/write utility")
        .arg(Arg::new("address")
            .short('a')
            .long("address")
            .help("Memory address to access (in hex)")
            .required(true))
        .arg(Arg::new("value")
            .short('v')
            .long("value")
            .help("Value to write (in hex)")
            .required(false))
        .get_matches();

}

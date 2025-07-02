use std::env;
use std::fs::File;
use clap::{Arg, ArgAction, Command};
use register_tool::unsafes::mmap_memory;
use rtoolconfig::RToolConfig;

mod register;
mod rtoolconfig;

// const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ;
const INDEX_MATCH: usize = 2 ;

const VALUE_MATCH: usize = 3 ;

/// todo consolodate with ucompleter
fn find_config_file(arg0: &str, env_var: &str) -> Result<String, String> {
    let home = env::var("HOME").unwrap_or("".to_string());
    let default_path = format!(".:{home}/.config/{arg0}:/etc/{arg0}");
    let path = env::var(env_var).unwrap_or(default_path);
    let paths: Vec<&str> = path.split(':').collect();
    let target = format!("{}.yaml", arg0) ;

    for path in paths {
        let file_path = format!("{}/{}", path, target);
        if std::path::Path::new(&file_path).exists() {
            return Ok(file_path);
        }
    }
    Err(format!("no config file not found for {}", arg0))
}


fn main() {
    let matches = Command::new("register_tool")
        .version("0.1.0")
        .author("Register Tool Developer")
        .about("Memory register read/write utility")
        .arg(Arg::new("file")
            .short('f').help("File of reg definitions, overriding defaults"))
        .arg(Arg::new("verbose")
            .short('v')
            .action(ArgAction::SetTrue)
            .required(false))
        .arg(Arg::new("dump")
            .short('d')
            .long("dump")
            .action(ArgAction::SetTrue)
            .help("Dump the properties of this register, do not set or read"))
        .arg(Arg::new("registers")
            .help("Register names to access")
            .required(true)
            .trailing_var_arg(true).num_args(1..))
        .get_matches();
    
    let path = find_config_file("register_tool", "REGISTER_TOOL_PATH").expect("no config file found");

    let config = RToolConfig::new(&mut File::open(path).expect("Failed to open config file"));

    let addr = if !matches.get_flag("dump") {
         mmap_memory(config.device.as_str(), config.base, config.length).expect("mmap failed")
    } else {
        0 as *mut u8
    };   // Close the if block for dump check

    if let Some(registers) = matches.get_many::<String>("registers") {
        for register in registers {
            let reg = config.get_register(&register).expect("register not found");

            if matches.get_flag("dump") {
               continue ;
            }
            if reg.isset {
                reg.set(addr);
            }
            else {
                println!("0x{:08x}", reg.get(addr));
            }
        }
    }
}

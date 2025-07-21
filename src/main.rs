use std::fs::File;
use std::io::Write;
use clap::{Arg, ArgAction, Command};
use register_tool::unsafes::mmap_memory;
use rtoolconfig::RToolConfig;
use aep_rust_common::find_config_file;
use aep_rust_common::find_config_file::find_config_file;
use std::process;
use aep_rust_common::descender::Descender;
use aep_rust_common::yaml_descender::YamlDescender;
use register_tool::register_tool::RegisterTool;

mod register;
mod rtoolconfig;

// const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ;
const INDEX_MATCH: usize = 2 ;

const VALUE_MATCH: usize = 3 ;




fn main() {
    let options = Command::new("register_tool")
        .version("0.1.0")
        .author("Register Tool Developer")
        .about("Memory register read/write utility")
        .arg(Arg::new("file")
            .short('f')
            .help("File of reg definitions, overriding defaults"))
        .arg(Arg::new("verbose")
            .short('v')
            .action(ArgAction::SetTrue)
            .required(false))
        .arg(Arg::new("test")
            .short('t')
            .long("test")
            .action(ArgAction::SetTrue)
            .help("Enable test mode")
            .required(false)
            .default_value("false"))
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

    let registers: Vec<&str> = options
        .get_many::<String>("registers")
        .expect("Required argument missing")
        .map(|s| s.as_str())
        .collect();



    let config_file = match options.get_one::<String>("file") {
        Some(s) => s,
        None => &{
            match find_config_file("register_tool", "REGISTER_TOOL_CONFIG") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(2);
                }
            }
        }
    } ;
    
    let descender = if config_file.ends_with(".yaml") {
        let yd  = match YamlDescender::new_from_file(config_file, true) {
            Ok(yd) => yd,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(2);
            }
        } ;
        Box::new(yd)
    } else if config_file.ends_with(".json") {
        todo!("JSON config file support")
    } else {
        eprintln!("uknonwn file type: {}", config_file);
        process::exit(3);
    } ;

    let mut register_tool = RegisterTool::new(descender);

    match register_tool.gather_regs(registers) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    } ;
    
    if *options.get_one::<bool>("test").unwrap_or(&false) {
        register_tool.set_test_area() ;
    } else {
        register_tool.set_base_address() ;
    }
    
    let results =  register_tool.apply_registers(|v| {
        println!("{}", v);
        Ok(v)
    }).unwrap() ;

    for r in results {
        match r {
            Ok(i) => {}
            Err(e) => {
                process::exit(1);
            }
        }
    }

    process::exit(0);
}

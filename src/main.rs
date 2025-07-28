// 
// SPDX-License-Identifier: MIT
// 
// Copyright (c) 2025 Andrew Ellis Page
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// 
use std::fs::File;
use std::io::Write;
use clap::{Arg, ArgAction, Command};
use aep_rust_common::find_config_file::find_config_file;
use std::process;
use aep_rust_common::yaml_descender::YamlDescender;
use register_tool::register_tool::RegisterTool;

mod register_op;

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

    let mut register_tool = match RegisterTool::new(descender) {
       Ok(rt) => rt,
        Err(errs) => {
            for e in errs {
                eprintln!("{}", e);
            }
            process::exit(4);
        }
    } ;


    if *options.get_one::<bool>("dump").unwrap_or(&false) {
        match register_tool.dump_registers(&registers) {
            Ok(_) =>         process::exit(0),
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    }

    /*
     * gather up all the registers to read or set.
     * If there are erroneous registers report them all and exit
     */
    match register_tool.gather_regs(&registers) {
        Ok(_) => {}
        Err(e) => {
            for e in e {
                eprintln!("{}", e);           
            }
            process::exit(1);
        }
    } ;

    if *options.get_one::<bool>("test").unwrap_or(&false) {
        register_tool.set_test_area() ;
    } else {
        match register_tool.set_base_address() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }       
        }
    }
    
    let results =  register_tool.apply_registers(|v| {
        println!("{}", v);
        Ok(v)
    }).unwrap() ;

    for r in results {
        match r {
            Ok(_) => {}
            Err(e) => {
                process::exit(1);
            }
        }
    }

    process::exit(0);
}

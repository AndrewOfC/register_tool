use std::env;
use std::fs::File;
use std::io::Read;
use clap::{Arg, ArgAction, Command};
use yaml_rust::{Yaml, YamlLoader};
use regex::Regex;
use register_tool::parse_bits;
use register_tool::unsafes::mmap_memory;

const  WHOLE_MATCH: usize = 0 ;
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

struct Register {
    pub offset: u64,
    pub set_mask: u32,
    pub clr_mask: u32,
    pub shift: u32,
    pub width: u32,
    pub isset: bool,
    value: u32,
}

impl Register {
    pub fn new(offset: u64, set_mask: u32, clr_mask: u32, shift: u32, width: u32, isset: bool, value: u32) -> Register {
        Register { offset, set_mask, clr_mask, shift, width, isset, value }
    }
    pub fn set(&self, addr: *mut u8) -> u32 {
        if self.value >= 0x01 << self.width { panic!("value out of range"); }

        let addr2 = unsafe { (addr as *mut u32).add(self.offset as usize) };
        let value = self.value << self.shift;
        unsafe {
            let curr_value = *addr2;
            *addr2 = (curr_value & !self.clr_mask) | (value & self.set_mask);
        }
        value
    }
}

struct RToolConfig {
    docs: Vec<Yaml>,
    re: Regex,

    pub device: String,
    pub base: u64,
    pub length: u64,
    registers_key: String
}

impl RToolConfig {
    /// ctor
    pub fn new<R: Read>(reader:&mut R) -> RToolConfig {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).expect("Failed to read config file");
        let docs = YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");
        let re = Regex::new(r"([^.\[\]=]+)|\[(\d+)]|=?(?:0x)?([0-9A-Fa-f]+)?$").unwrap() ;
        
        let base = docs[0]["base"].as_i64().expect("base not found") as u64;
        let length = docs[0]["length"].as_i64().expect("length not found") as u64;

        //let key = docs[0]["completion-metadata"]["root"].as_str().unwrap_or("registers");
        let device = docs[0]["device"].as_str().unwrap_or("/dev/mem").to_string();
        
        RToolConfig { device: device, re: re , docs, base: base, length: length, registers_key: "registers".to_string()}
    }
    
    pub fn get_register(&self, name: &str) -> Result<Register, String> {
        let mut current = &self.docs[0][self.registers_key.as_str()] ;
        let captures = self.re.captures(name).expect("invalid register name");
        
        let mut value = 0 ;
        let mut isset = false ;
        for part in self.re.captures_iter(name) {
            (value, isset) = if let Some(v) = part.get(VALUE_MATCH) {
                (v.as_str().parse::<u32>().unwrap(), true)
            }
            else {
                (0, false)
            } ;
            let key = if let Some(k) = part.get(KEY_MATCH) {
                current = &current[k.as_str()] ;
            } else if let Some(idx) = part.get(INDEX_MATCH) {
                current = &current[idx.as_str().parse::<usize>().unwrap()] ;
            } ;
            
        }
        
        let(width, mask, lo) = parse_bits(&current["bits"].as_str().unwrap())? ;

        let r = Register::new(current["offset"].as_i64().expect("offset not found") as u64,
                              mask,
                              mask,
                              lo,
                              width, isset, value);
        Ok(r)

    }
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
         unsafe { mmap_memory(config.device.as_str(), config.base, config.length) }.expect("mmap failed")
    } else {
        println!("no set") ;
        0 as *mut u8
    };   // Close the if block for dump check

    if let Some(registers) = matches.get_many::<String>("registers") {
        for register in registers {
            let reg = config.get_register(&register).expect("register not found");
            // {
            //     let mut out_str = String::new();
            //     let mut emitter = YamlEmitter::new(&mut out_str);
            //     emitter.dump(reg).unwrap();
            // }
            // println!("{}", out_str);
            println!("offset={} shift={}", reg.offset, reg.shift) ;
            if !matches.get_flag("dump") {
                reg.set(addr) ;
            }
        }
    }



}

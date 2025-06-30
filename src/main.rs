use std::env;
use std::fs::File;
use std::io::Read;
use clap::{Command, Arg};
use yaml_rust::{YamlLoader, Yaml};
use regex::Regex;

const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ;
const INDEX_MATCH: usize = 2 ;


/// todo consolodate with ucompleter
fn find_config_file(arg0: &str, env_var: &str) -> Result<String, String> {
    let home = env::var("HOME").unwrap_or("".to_string());
    let default_path = format!(".:{}/.config/register_tool:/etc/register_tool", home);
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

struct RToolConfig<'a> {
    docs: Vec<Yaml>,
    re: Regex,
    registers: &'a Yaml,
    
    base: u64,
    length: u64
}

impl RToolConfig<'_> {
    /// ctor
    pub fn new<R: Read>(reader:&mut R) -> RToolConfig {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).expect("Failed to read config file");
        let docs = YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");
        let re = Regex::new(r"([^\.\[\]]+)|(?:(?:\[(\d+)\]))").unwrap() ;
        let config = &docs[0];
        
        let base = config["base"].as_i64().expect("base not found") as u64;
        let length = config["length"].as_i64().expect("length not found") as u64;


        let mut registers_key = "registers";
        let yroot = &config["completion-metadata"]["root"] ;
        if !matches!(yroot, Yaml::BadValue) {
            registers_key = yroot.as_str().unwrap();
        }
        let registers = &config[registers_key];
        assert!(!matches!(current, Yaml::BadValue), format!("{}} section not found in config", registers_key));

        RToolConfig { re: re , docs: docs, base: base, length: length, registers: registers}
    }
    
    pub fn get_register(&self, name: &str) -> Result<&Yaml, String> {
        let mut current = self.registers;
        let captures = self.re.captures(name).expect("invalid register name");
        if  captures.len() !=3  {
            panic!("invalid register name: {}", name);
        }
        
        for part in self.re.captures_iter(name) {
            let key = if let Some(k) = part.get(KEY_MATCH) {
                current = &current[k.as_str()] ;
            } else if let Some(idx) = part.get(INDEX_MATCH) {
                current = &current[idx.as_str().parse::<usize>().unwrap()] ;
            } else {
                panic!("No valid key or index found")
            };
        }

        assert!(!matches!(current["offset"], Yaml::BadValue), "Register does not have required 'offset' field");

        Ok(current)
    }
}


fn main() {
    let matches = Command::new("register_tool")
        .version("0.1.0")
        .author("Register Tool Developer")
        .about("Memory register read/write utility")
        .arg(Arg::new("file")
            .short('f').help("File of reg definitions, overriding defaults"))
        .arg(Arg::new("value")
            .short('v')
            .long("value")
            .help("Value to write (in hex)")
            .required(false))
        .arg(Arg::new("dump")
            .short('d')
            .help("Dump the properties of this register"))
        .arg(Arg::new("registers")
            .help("Register names to access")
            .required(true)
            .trailing_var_arg(true).num_args(1..))
        .get_matches();
    
    let path = find_config_file("register_tool", "REGISTER_TOOL_PATH").expect("no config file found");

    let config = RToolConfig::new(&mut File::open(path).expect("Failed to open config file"));
    
    if let Some(registers) = matches.get_many::<String>("registers") {
        for register in registers {
            println!("Processing register: {}", register);
        }
    }



}

use regex::Regex;
use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;
use yaml_rust::{Yaml, YamlLoader};

pub struct RToolConfig {
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

    pub fn get_value<T>(root: &Yaml, current: &Yaml, name: &str) -> T where T: std::str::FromStr, <T as FromStr>::Err: Debug {
        if !current["parent"].is_badvalue()  {
            return Self::get_value(root,&root["parent"], name) ;
        }
        T::from_str(current[name].as_str().unwrap()).unwrap()
    }

    
}
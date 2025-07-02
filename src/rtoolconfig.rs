use yaml_rust::{Yaml, YamlLoader};
use regex::Regex;
use std::io::Read;
use register_tool::parse_bits;
use crate::register::Register;
use crate::{INDEX_MATCH, KEY_MATCH, VALUE_MATCH};

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

    pub fn get_register(&self, name: &str) -> Result<Register, String> {
        let mut current = &self.docs[0][self.registers_key.as_str()] ;
        let mut value = 0 ;
        let mut isset = false ;
        for part in self.re.captures_iter(name) {
            (value, isset) = if let Some(v) = part.get(VALUE_MATCH) {
                (v.as_str().parse::<u32>().unwrap(), true)
            }
            else {
                (0, false)
            } ;
            if let Some(k) = part.get(KEY_MATCH) {
                current = &current[k.as_str()] ;
            } else if let Some(idx) = part.get(INDEX_MATCH) {
                current = &current[idx.as_str().parse::<usize>().unwrap()] ;
            } ;

        }

        let(mask,  lo) = parse_bits(&current["bits"].as_str().unwrap())? ;

        let r = Register::new(current["offset"].as_i64().expect("offset not found") as u64,
                              mask ^ 0xFFFFFFFF,
                              mask,
                              lo,
                              isset, value);
        Ok(r)

    }
}
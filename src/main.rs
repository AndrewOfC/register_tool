use std::env;
use std::fs::File;
use std::io::Read;
use clap::{Command, Arg};
use yaml_rust::YamlLoader;
/// todo consolodate with ucomplet4er
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

    let mut contents = String::new();
    let mut file = File::open(&path).expect("Failed to open config file");
    file.read_to_string(&mut contents).expect("Failed to read config file");

    let docs = YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");
    let config = &docs[0];

    if let Some(registers) = matches.get_many::<String>("registers") {
        for register in registers {
            println!("Processing register: {}", register);
        }
    }
}

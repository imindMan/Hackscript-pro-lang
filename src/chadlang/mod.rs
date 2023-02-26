// ChadLang is an interpreter engine that provides you tools to 
// build your own interpreted programming language

// Idea:
// This engine came up to my mind when I think about interpreted llvm
// It should be called "illvm", but, "chadlang" is the final name
//
// Let's start with this file

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


// WHAT: Declare an Interpreter Struct for runtime engine
pub struct Interpreter {
    // WHAT INSIDE:
    // - Input string: the string to run
    // - power? (bool): check that if the engine is started
    
    input_string: String,
    power: bool,

}

impl Interpreter {
    // This function will initialize a new Interpreterengine
    pub fn new() -> Interpreter {
        
        Interpreter {
            input_string: String::from(""),
            power: false,
        }

    }
    // This function will initialize all the json config file (followed by the given path)
    // and then config it into this engine
    pub fn config(&self, _path: &str) -> String {
        let path = Path::new(_path);
        
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
            Ok(s) => s,
        };
        let s_slice: &str = &s[..];
        let json: serde_json::Value =
            serde_json::from_str(s_slice).expect("JSON was not well-formatted");
        println!("{:?}", json);
        s
    }
}


// WHAT: Declare a config struct for configuration


struct Config {
    // general configuration
    auto_start: bool,
    log: bool,
}

impl Config {
    // this function will initialize a new config
    

}

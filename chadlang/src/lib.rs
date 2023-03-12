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
use serde_json::Value;

pub mod logger;
use logger::Logger;
use lexer::Lexer;

// WHAT: Declare an Interpreter Struct for runtime engine
pub struct Interpreter {
    // WHAT INSIDE:
    // - Input string: the string to run
    // - power? (bool): check that if the engine is started
    
    input_string: String,
    power: bool,
    config: Config
}

impl Interpreter {
    // This function will initialize a new Interpreterengine
    pub fn new(string: String) -> Interpreter {
        
        Interpreter {
            input_string: string,
            power: false,
            config: Config::new()
        }

    }
    // This function will initialize all the json config file (followed by the given path)
    // and then config it into this engine
    pub fn config(&mut self, _path: &str) {
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
        // read the json data
        let s_slice: &str = &s[..];
        let json: Value = serde_json::from_str(s_slice).expect("JSON formatted wrong");
        // start parsing the attributes
        match json["auto_start"] {
            Value::Bool(true) => {self.config.auto_start = true; self.power = true},
            _ => (),
        }
        match json["log"] {
            Value::Bool(true) => {self.config.log = true;},
            _ => (),
        }
    }
    // This function will start the engine 
    pub fn start(&mut self) {
        if self.config.log == false {
            match self.power {
                false => self.power = true,
                true => panic!("This machine has already started!"),
            }
        }
        else {
            let _logger = Logger::new();
            match self.power {
                false => {
                    _logger.show_process("Power up..."); 
                    self.power = true; 
                    _logger.show_success("Successfully powered up!");
                },
                true => {
                    _logger.show_error("Error when checking! Panic starting...");
                    panic!("This machine has already started!"); 
                },
            }
        }
    }
    // This function will end the engine 
    pub fn end(&mut self) {
        if self.config.log == false {
            match self.power {
                true => self.power = false,
                false => panic!("This machine has already ended!"),
            }
        }
        else {
            let _logger = Logger::new();
            match self.power {
                true => {
                    _logger.show_process("Shut down..."); 
                    self.power = true; 
                    _logger.show_success("Successfully shut down!");
                },
                false => {
                    _logger.show_error("Error when checking! Panic starting...");
                    panic!("This machine has already ended!"); 
                },
            }
        }

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
    pub fn new() -> Config {
        Config {
            // default settings 
            auto_start: false,
            log: false,
        }
    } 

}

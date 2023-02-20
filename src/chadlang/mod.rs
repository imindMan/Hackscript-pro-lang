// ChadLang is an interpreter engine that provides you tools to 
// build your own interpreted programming language

// Idea:
// This engine came up to my mind when I think about interpreted llvm
// It should be called "illvm", but, "chadlang" is the final name
//
// Let's start with this file


// WHAT: Declare an Interpreter Struct for runtime engine

pub struct Interpreter {
    // WHAT INSIDE:
    // - Input string: the string to run
    // - power? (bool): check that if the engine is started
    
    input_string: String,
    power: bool,

}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            input_string: String::from(""),
            power: false,
        }

    }

}

/* 
 * First implementation for the Logger tool, one of the utility of chadlang
 * */

// define Logger, first empty
#[derive(Debug)]
pub struct Logger {
    log: bool
}

impl Logger {
    
    pub fn config(&mut self, signal: bool) {
        self.log = signal;
    }
    // this function will create a Logger struct
    pub fn new() -> Logger {

        Logger {

            log: true,
        }
    }
    // show the error message
    pub fn show_error(&self, error: &str) {
        if self.log == true {
            println!("[!] {error}");
        }
    }
    // show the success point of doing something
    pub fn show_success(&self, success: &str) {
        if self.log == true {
            println!("[y] {success}");
        }
    }
    // show the process of doing something
    pub fn show_process(&self, process: &str) {
        if self.log == true {
            println!("-> {process}");
        }
    }

}



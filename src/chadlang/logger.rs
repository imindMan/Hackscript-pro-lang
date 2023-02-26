/* 
 * First implementation for the Logger tool, one of the utility of chadlang
 * */

// define Logger, first empty
pub struct Logger {
    
}

impl Logger {

    // this function will create a Logger struct
    pub fn new() -> Logger {

        Logger {


        }
    }
    // show the error message
    pub fn show_error(&self, error: &str) {
        println!("[!] {error}");
    }
    // show the success point of doing something
    pub fn show_success(&self, success: &str) {

        println!("[y] {success}");
    }
    // show the process of doing something
    pub fn show_process(&self, process: &str) {
        println!("-> {process}");
    }

}

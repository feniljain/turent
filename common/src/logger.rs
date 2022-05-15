use std::error::Error;

#[derive(Clone)]
pub struct Logger {
    debug: bool,
}

impl Logger {
    pub fn new(debug: bool) -> Self {
        Logger { debug }
    }

    pub fn log_debug(&self, log: &str) {
        if self.debug {
            println!("{:?}", log);
        }
    }

    pub fn log(&self, log: String) {
        println!("{:?}", log);
    }
    pub fn log_err(&self, err: &impl Error) {
        println!("ERR: {:?}", err);
    }
}

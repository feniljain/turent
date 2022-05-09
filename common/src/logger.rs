#[derive(Clone)]
pub struct Logger {
    debug: bool,
}

impl Logger {
    pub fn new(debug: bool) -> Self {
        Logger { debug }
    }

    pub fn log_debug(&self, log: String) {
        if self.debug {
            println!("{:?}", log);
        }
    }

    pub fn log(&self, log: String) {
        println!("{:?}", log);
    }
}

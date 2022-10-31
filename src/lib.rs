extern crate serial;

pub struct BleuIO {
    pub port: String,
    pub baudrate: u32,
    pub timeout: u32,
    pub debug: bool,
}

impl BleuIO {
    pub fn new(port: String, baudrate: u32, timeout: u32, debug: bool) -> BleuIO {
        BleuIO {
            port, baudrate, timeout, debug
        }
    }

}

use crate::BleuIO;
use std::thread;
use std::time::Duration;

impl BleuIO {
    pub fn at(&mut self) -> Result<String, serial::Error> {
        let cmd = "AT";

        self.send_command(cmd).unwrap();

        let mut buf: String = String::new();
        thread::sleep(Duration::from_millis(50));
        // self.serial.read_to_string(&mut buf)?;
        println!("{}", buf);

        Ok("lol".to_owned())
    }
}

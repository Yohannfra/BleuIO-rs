extern crate serial;

use serial::prelude::*;
use std::error::Error;
use std::io::prelude::*;
use std::time::Duration;

pub struct BleuIO {
    pub port: String,
    pub baudrate: serial::core::BaudRate,
    pub timeout: u64,
    pub debug: bool,
}

impl BleuIO {
    pub fn new(port: &str, timeout: u64, debug: bool) -> BleuIO {
        BleuIO {
            port: port.to_owned(),
            baudrate: serial::Baud57600,
            timeout,
            debug,
        }
    }

    pub fn connect(&mut self) -> Result<(), serial::Error> {
        print!(
            "Opening port {} with baudrate {:?} ... ",
            self.port, self.baudrate
        );
        let mut port = serial::open(&self.port).unwrap();
        println!("OK");

        port.reconfigure(&|settings| {
            settings.set_baud_rate(self.baudrate)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(self.timeout))?;

        let cmd = "AT\r\n";
        println!("writing bytes: {:?}", cmd);
        port.write(cmd.as_bytes())?;

        // std::thread::sleep(Duration::from_millis(100));

        port.flush()?;

        let mut buf: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        println!("reading bytes");
        port.read(&mut buf[..])?;
        port.read(&mut buf[..])?;
        println!("{:?}", buf);

        Ok(())
    }
}

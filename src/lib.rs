extern crate serial;

use serial::prelude::*;
use serial::SystemPort;
use std::io::{Read, Write};
use std::time::Duration;

use std::sync::atomic;
use atomic::{Ordering, AtomicBool};
use std::sync::Arc;
use std::thread;

pub struct BleuIO {
    serial: SystemPort,
    pub port: String,
    pub baudrate: serial::core::BaudRate,
    pub timeout: u64,
    pub debug: bool,

    is_running: Arc<AtomicBool>,
    rx_thread: Option<thread::JoinHandle<()>>,
    tx_thread: Option<thread::JoinHandle<()>>,
}

impl BleuIO {
    pub fn new(port: &str, timeout: u64, debug: bool) -> BleuIO {
        BleuIO {
            serial: serial::open(port).unwrap(),
            port: port.to_owned(),
            baudrate: serial::Baud57600,
            timeout,
            debug,
            is_running: Arc::new(AtomicBool::new(false)),
            rx_thread: None,
            tx_thread: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), serial::Error> {
        self.serial.reconfigure(&|settings| {
            settings.set_baud_rate(self.baudrate)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.serial
            .set_timeout(Duration::from_millis(self.timeout))?;

        Ok(())
    }

    pub fn start_daemon(&mut self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err("Thread already started".to_owned());
        }

        self.is_running.store(true, Ordering::Relaxed);

        let is_running = self.is_running.clone();

        self.rx_thread = Some(thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {

            }
        }));
        

        let is_running = self.is_running.clone();
        self.tx_thread = Some(thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {

            }
        }));
        Ok(())
    }

    pub fn stop_daemon(&mut self) -> Result<(), String> {
        if !self.is_running.load(Ordering::Relaxed) {
            return Err("Thread isn't started".to_owned());
        }

        self.is_running.store(false, Ordering::Relaxed);
        // self.tx_thread.as_ref().unwrap().join();
        // self.rx_thread.as_ref().unwrap().join();

        Ok(())
    }

    pub fn at(&mut self) -> Result<String, serial::Error> {
        let cmd = "AT";

        self.send_command(cmd).unwrap();

        let mut buf: String = String::new();
        thread::sleep(Duration::from_millis(50));
        self.serial.read_to_string(&mut buf)?;
        println!("{}", buf);

        Ok("lol".to_owned())
    }

    fn set_echo(&mut self, state: bool) -> Result<(), serial::Error> {
        if !state {
            self.send_command("ATE0")?;
        } else {
            self.send_command("ATE1")?;
        }
        thread::sleep(Duration::from_millis(50));

        Ok(())
    }

    fn set_verbose(&mut self, state: bool) -> Result<(), serial::Error> {
        if !state {
            self.send_command("ATV0")?;
        } else {
            self.send_command("ATV1")?;
        }
        thread::sleep(Duration::from_millis(50));

        Ok(())
    }

    fn task_tx(&mut self) {}

    fn send_command(&mut self, cmd: &str) -> Result<(), serial::Error> {
        let full_cmd: String = cmd.to_owned() + "\r\n";

        self.serial.write(full_cmd.as_bytes())?;

        Ok(())
    }
}

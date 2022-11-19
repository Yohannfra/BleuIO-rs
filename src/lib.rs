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

    threads_running: Arc<AtomicBool>,
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
            threads_running: Arc::new(AtomicBool::new(false)),
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
        if self.threads_running.load(Ordering::Relaxed) {
            return Err("Thread already started".to_owned());
        }
        println!("Starting daemon");

        self.threads_running.store(true, Ordering::Relaxed);

        let threads_running = self.threads_running.clone();

        self.rx_thread = Some(thread::spawn(move || {
            while threads_running.load(Ordering::Relaxed) {
                println!("rx running");
                std::thread::sleep(std::time::Duration::from_millis(900));
            }
            println!("Ending rx thread");
        }));


        let threads_running = self.threads_running.clone();
        self.tx_thread = Some(thread::spawn(move || {
            while threads_running.load(Ordering::Relaxed) {
                println!("tx running");
                std::thread::sleep(std::time::Duration::from_millis(850));
            }
            println!("Ending tx thread");
        }));

        Ok(())
    }

    pub fn stop_daemon(&mut self) -> Result<(), String> {
        if !self.threads_running.load(Ordering::Relaxed) {
            return Err("Thread isn't started".to_owned());
        }
        println!("Stopping daemon");

        self.threads_running.store(false, Ordering::Relaxed);
        self.tx_thread.take().expect("tx_take failed").join().expect("Could not join tx thread");
        self.rx_thread.take().expect("rx_take failed").join().expect("Could not join rx thread");

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

extern crate serial;

mod rx_thread;
mod tx_thread;

mod commands;

use serial::prelude::*;
use serial::SystemPort;
use std::io::{Read, Write};
use std::time::Duration;

use atomic::{AtomicBool, Ordering};
use std::sync::atomic;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct BleuIO {
    serial: Arc<Mutex<SystemPort>>,
    pub port: String,
    pub timeout: u64,
    pub debug: bool,

    threads_running: Arc<AtomicBool>,
    rx_thread: Option<thread::JoinHandle<()>>,
    rx_state_ready: Arc<AtomicBool>,
    rx_buffer: Arc<Mutex<String>>,

    tx_state_ready: Arc<AtomicBool>,
    tx_thread: Option<thread::JoinHandle<()>>,
    tx_buffer: Arc<Mutex<String>>,
}

impl BleuIO {
    pub fn new(port: &str, timeout: u64, debug: bool) -> BleuIO {
        BleuIO {
            serial: Arc::new(Mutex::new(serial::open(port).unwrap())),
            port: port.to_owned(),
            timeout,
            debug,
            threads_running: Arc::new(AtomicBool::new(false)),

            rx_thread: None,
            rx_state_ready: Arc::new(AtomicBool::new(true)),
            rx_buffer: Arc::new(Mutex::new(String::new())),

            tx_thread: None,
            tx_state_ready: Arc::new(AtomicBool::new(false)),
            tx_buffer: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn connect(&mut self) -> Result<(), serial::Error> {
        self.serial
            .clone()
            .lock()
            .unwrap()
            .reconfigure(&|settings| {
                settings.set_baud_rate(serial::Baud57600)?;
                settings.set_char_size(serial::Bits8);
                settings.set_parity(serial::ParityNone);
                settings.set_stop_bits(serial::Stop1);
                settings.set_flow_control(serial::FlowNone);
                Ok(())
            })?;

        self.serial
            .clone()
            .lock()
            .unwrap()
            .set_timeout(Duration::from_millis(self.timeout))?;

        Ok(())
    }

    pub fn start_daemon(&mut self) -> Result<(), String> {
        if self.threads_running.load(Ordering::Relaxed) {
            return Err("Thread already started".to_owned());
        }
        println!("Starting daemon");

        self.threads_running.store(true, Ordering::Relaxed);
        self.run_rx_thread();
        self.run_tx_thread();

        self.set_echo(false).unwrap();

        Ok(())
    }

    pub fn stop_daemon(&mut self) -> Result<(), String> {
        if !self.threads_running.load(Ordering::Relaxed) {
            return Err("Thread isn't started".to_owned());
        }
        println!("Stopping daemon");

        self.threads_running.store(false, Ordering::Relaxed);
        self.tx_thread
            .take()
            .expect("tx_take failed")
            .join()
            .expect("Could not join tx thread");
        self.rx_thread
            .take()
            .expect("rx_take failed")
            .join()
            .expect("Could not join rx thread");

        Ok(())
    }

    fn send_command(&mut self, cmd: &str) -> Result<(), serial::Error> {
        let full_cmd: String = cmd.to_owned() + "\r\n";

        *self.tx_buffer.lock().unwrap() = full_cmd.to_owned();
        self.tx_state_ready.store(true, Ordering::Relaxed);

        Ok(())
    }
}

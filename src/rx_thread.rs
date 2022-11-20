use super::BleuIO;

use atomic::Ordering;
use std::io::Read;
use std::sync::atomic;
use std::thread;

impl BleuIO {
    pub fn run_rx_thread(&mut self) {
        let threads_running = self.threads_running.clone();
        let serial_con = self.serial.clone();
        let rx_buf = self.rx_buffer.clone();

        self.rx_thread = Some(thread::spawn(move || {
            while threads_running.load(Ordering::Relaxed) {
                println!("rx running");

                let mut buf: Vec<u8> = vec![0; 1000];

                let bytes_read = match serial_con.clone().lock().unwrap().read(buf.as_mut_slice()) {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        continue;
                    }
                };

                let s = std::str::from_utf8(&buf[..bytes_read]);

                if s.is_ok() {
                    println!("Read {}", s.as_ref().unwrap());
                    *rx_buf.lock().unwrap() = s.unwrap().to_owned();
                } else {
                    eprintln!("Read invalid data: {:?}", &buf[..bytes_read]);
                }
            }
            println!("Ending rx thread");
        }));
    }
}

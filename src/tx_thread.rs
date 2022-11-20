use super::BleuIO;
use atomic::Ordering;
use std::io::Write;
use std::sync::atomic;
use std::thread;

impl BleuIO {
    pub fn run_tx_thread(&mut self) {
        let threads_running = self.threads_running.clone();
        let serial_con = self.serial.clone();
        let tx_ready = self.tx_state_ready.clone();
        let tx_buf = self.tx_buffer.clone();

        self.tx_thread = Some(thread::spawn(move || {
            while threads_running.load(Ordering::Relaxed) {
                if tx_ready.load(Ordering::Relaxed) {
                    let a = tx_buf.lock().unwrap();
                    serial_con
                        .clone()
                        .lock()
                        .unwrap()
                        .write(a.as_bytes())
                        .unwrap();
                    tx_ready.store(false, Ordering::Relaxed);
                }
            }
            println!("Ending tx thread");
        }));
    }
}

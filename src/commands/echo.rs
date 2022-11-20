use crate::BleuIO;
use std::thread;
use std::time::Duration;

impl BleuIO {
    pub fn set_echo(&mut self, state: bool) -> Result<(), serial::Error> {
        if !state {
            self.send_command("ATE0")?;
        } else {
            self.send_command("ATE1")?;
        }
        thread::sleep(Duration::from_millis(50));

        Ok(())
    }
}

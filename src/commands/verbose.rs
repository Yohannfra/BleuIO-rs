use crate::BleuIO;
use std::thread;
use std::time::Duration;

impl BleuIO {
    fn set_verbose(&mut self, state: bool) -> Result<(), serial::Error> {
        if !state {
            self.send_command("ATV0")?;
        } else {
            self.send_command("ATV1")?;
        }
        thread::sleep(Duration::from_millis(50));

        Ok(())
    }
}

use std::{io::{stdout, Write}, thread::sleep, time::Duration};

use english_clock::Clock;

fn main() {
    let mut clock = Clock::new(6, 0);
    loop {
        clock.refresh();
        print!("\rIt's {} {}        ", clock.get_hour(), clock.get_minute());
		stdout().flush().unwrap();
        sleep(Duration::from_secs(5));
    }
}

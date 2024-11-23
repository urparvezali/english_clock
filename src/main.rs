use std::{thread::sleep, time::Duration};

use english_clock::Clock;

fn main() {
    let mut clock = Clock::new(6, 0);

    loop {
        clock.refresh();
        println!("It's {} {}", clock.get_hour(), clock.get_minute());
        sleep(Duration::from_secs(5));
    }
}

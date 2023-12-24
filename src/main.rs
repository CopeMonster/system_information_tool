mod systeminfo;
mod utils;

use std::{thread, time};

fn main() {
    loop {
        let system = systeminfo::SystemInfo::new();

        utils::clear_terminal();
        system.display();

        thread::sleep(time::Duration::from_secs(1));
    }
}


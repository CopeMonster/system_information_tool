use sysinfo::System;
use text_colorizer::Colorize;
use crate::utils::bytes_to_mbytes;

pub struct MemoryInfo {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
}

impl MemoryInfo {
    pub fn new() -> MemoryInfo {
        let mut system = System::new_all();

        system.refresh_all();

        let total_memory = bytes_to_mbytes(system.total_memory());
        let used_memory = bytes_to_mbytes(system.used_memory());
        let free_memory = bytes_to_mbytes(system.free_memory());

        MemoryInfo {
            total_memory,
            used_memory,
            free_memory,
        }
    }

    pub fn display(&self) {
        println!("Total memory: {} Mb", self.total_memory.to_string().bright_red());
        println!("Used memory: {} Mb", self.used_memory.to_string().bright_red());
        println!("Free memory: {} Mb", self.free_memory.to_string().bright_red());
    }
}
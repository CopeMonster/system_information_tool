use sysinfo::System;
use text_colorizer::Colorize;

pub struct CPUInfo {
    cpu_usage: f32,
    number_of_cores: usize,
    cpu_frequency: u64,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        let mut system = System::new_all();

        system.refresh_all();

        let cpu_usage = system.global_cpu_info().cpu_usage();
        let number_of_cores = system.cpus().len();
        let cpu_frequency = system.cpus()[0].frequency();

        CPUInfo {
            cpu_usage,
            number_of_cores,
            cpu_frequency
        }
    }

    pub fn display(&self) {
        println!("CPU Usage: {:.2}%", self.cpu_usage.to_string().bright_red());
        println!("Number of Cores: {}", self.number_of_cores.to_string().bright_red());
        println!("CPU Frequency: {} MHz", self.cpu_frequency.to_string().bright_red());
    }
}
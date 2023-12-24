use self::{cpu::CPUInfo, memory::MemoryInfo, disk::DisksInfo};

mod cpu;
mod memory;
mod disk;

pub struct SystemInfo {
    cpu: CPUInfo,
    memory: MemoryInfo,
    disks: DisksInfo
}

pub trait Display {
    fn display();
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            cpu: CPUInfo::new(),
            memory: MemoryInfo::new(),
            disks: DisksInfo::new(),
        }
    }

    pub fn display(&self) {
        self.cpu.display();
        self.memory.display();
        self.disks.display();
    }
}

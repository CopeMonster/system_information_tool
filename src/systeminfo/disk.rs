use sysinfo::Disks;
use text_colorizer::Colorize;
use crate::utils::bytes_to_mbytes;

pub struct DiskInfo {
    disk_name: String,
    total_space: u64,
    available_space: u64,
}

pub struct DisksInfo {
    disks: Vec<DiskInfo>
}

impl DiskInfo {
    pub fn new(disk: &sysinfo::Disk) -> DiskInfo {
        let total_space = bytes_to_mbytes(disk.total_space());
        let available_space = bytes_to_mbytes(disk.available_space());
        let disk_name = disk.name().to_string_lossy().into_owned();

        DiskInfo {
            total_space,
            available_space,
            disk_name,
        }
    }

    pub fn display(&self) {
        println!("Disk Name: {}", self.disk_name.to_string().bright_red());
        println!("Total Disk Space: {} MiB", self.total_space.to_string().bright_red());
        println!("Available Disk Space: {} MiB", self.available_space.to_string().bright_red());
    }
}

impl DisksInfo {
    pub fn new() -> DisksInfo {
        let mut diskinfos: Vec<DiskInfo> = vec![];

        for disk in &Disks::new_with_refreshed_list() {
            diskinfos.push(DiskInfo::new(&disk));
        }

        DisksInfo {
            disks: diskinfos,
        }
    }

    pub fn display(&self) {
        for disk in &self.disks {
            disk.display();
        }
    }
}
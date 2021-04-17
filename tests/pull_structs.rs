/*
 * Copyright (C) 2021  Oakes, Gregory C. <gregcoakes@gmail.com>
 * Author: Oakes, Gregory C. <gregcoakes@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation, either
 * version 3 of the License, or any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program.  If not, see
 * <http://www.gnu.org/licenses/>.
 */

use std::{
    fs,
    io::{BufRead, BufReader},
    process::Command,
};

use nvme::*;

fn strip_namespace<T: AsRef<str>>(dev: T) -> Option<String> {
    let ref_dev = dev.as_ref();
    // If it contains "nvme", then split at the first
    // occurance of 'n' from the right.
    ref_dev.find("nvme").map(|_| {
        // SAFETY: It must have at least one 'n'.
        let ctrl = ref_dev.rsplitn(2, 'n').nth(1).unwrap();
        // If the ctrl actually just split on the 'n' from
        // "nvme", then return the whole device.
        if ctrl.len() < 5 { ref_dev } else { ctrl }.to_string()
    })
}

fn get_dev() -> String {
    let mounted_devs: Vec<String> = fs::File::open("/proc/mounts")
        .map(BufReader::new)
        .ok()
        .map(|mounts| {
            mounts
                .lines()
                // Remove I/O errored lines.
                .filter_map(Result::ok)
                // Filter out non-nvme devices and strip namespaces/partitions from it.
                .filter_map(|l| {
                    // First word of the line is the device.
                    l.split_whitespace().nth(0).and_then(strip_namespace)
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);
    let dev = fs::read_dir("/dev/spdk")
        .or_else(|_| fs::read_dir("/dev"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path().to_string_lossy().to_string())
        .filter(|dev| {
            dev.find("nvme").is_some()
                && !mounted_devs.iter().any(|mounted| dev.starts_with(mounted))
        })
        .filter_map(strip_namespace)
        .nth(0)
        .unwrap_or_else(|| "/dev/nvme0".to_string());
    println!("device: {}", dev);
    dev
}

#[ignore]
#[test]
fn pull_decode_smart() {
    let output = Command::new("nvme")
        .args(&["smart-log", "-o", "binary", get_dev().as_str()])
        .output()
        .expect("failed to pull smart log");
    assert!(output.status.success());
    println!(
        "{}",
        serde_json::to_string_pretty(SmartLog::from_bytes(output.stdout.as_slice())).unwrap()
    );
}

#[ignore]
#[test]
fn pull_decode_fw_log() {
    let output = Command::new("nvme")
        .args(&["fw-log", "-o", "binary", get_dev().as_str()])
        .output()
        .expect("failed to pull fw log");
    assert!(output.status.success());
    println!(
        "{}",
        serde_json::to_string_pretty(FwSlotLog::from_bytes(output.stdout.as_slice())).unwrap()
    );
}

#[ignore]
#[test]
fn pull_decode_err_log() {
    let output = Command::new("nvme")
        .args(&["error-log", "-e", "8", "-o", "binary", get_dev().as_str()])
        .output()
        .expect("failed to pull err log");
    assert!(output.status.success());
    println!(
        "{}",
        serde_json::to_string_pretty(<[ErrLogEntry]>::from_bytes(output.stdout.as_slice()))
            .unwrap()
    );
}

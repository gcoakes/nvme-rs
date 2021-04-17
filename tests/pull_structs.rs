/*
 * Copyright (C) 2021  Oakes, Gregory C. <gregcoakes@gmail.com>
 * Author: Oakes, Gregory C. <gregcoakes@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation, either
 * version 3 of the License, or (at your option) any later version.
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

use std::{env, process::Command};

use nvme::*;

#[ignore]
#[test]
fn pull_decode_smart() {
    println!("{:?}", env::var("OUT_DIR"));
    let output = Command::new("nvme")
        .args(&["smart-log", "-o", "binary", "/dev/nvme0"])
        .output()
        .expect("failed to pull smart log");
    assert!(output.status.success());
    println!(
        "{:?}",
        serde_json::to_string(SmartLog::from_bytes(output.stdout.as_slice()))
    );
}

#[ignore]
#[test]
fn pull_decode_fw_log() {
    println!("{:?}", env::var("OUT_DIR"));
    let output = Command::new("nvme")
        .args(&["fw-log", "-o", "binary", "/dev/nvme0"])
        .output()
        .expect("failed to pull fw log");
    assert!(output.status.success());
    println!(
        "{:?}",
        serde_json::to_string(FwSlotLog::from_bytes(output.stdout.as_slice()))
    );
}

#[ignore]
#[test]
fn pull_decode_err_log() {
    println!("{:?}", env::var("OUT_DIR"));
    let output = Command::new("nvme")
        .args(&["error-log", "-e", "64", "-o", "binary", "/dev/nvme0"])
        .output()
        .expect("failed to pull err log");
    assert!(output.status.success());
    println!(
        "{}",
        serde_json::to_string_pretty(<[ErrLogEntry]>::from_bytes(output.stdout.as_slice()))
            .unwrap()
    );
}

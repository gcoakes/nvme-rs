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

use crate::TransmuteSafe;

use modular_bitfield::prelude::*;

#[test_structure(size = 4096)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdNmsp {
    #[loc(0:7)]
    pub nsze: u64,
    #[loc(8:15)]
    pub ncap: u64,
    #[loc(16:23)]
    pub nuse: u64,
    #[loc(24:24)]
    pub nsfeat: Nsfeat,
    #[loc(25:25)]
    pub nlbaf: u8,
    #[loc(26:26)]
    pub flbas: Flbas,
    #[loc(27:27)]
    pub mc: Mc,
    #[loc(28:28)]
    pub dpc: Dpc,
    #[loc(30:30)]
    pub nmic: Nmic,
    #[loc(31:31)]
    pub rescap: Rescap,
    #[loc(32:32)]
    pub fpi: Fpi,
    #[loc(33:33)]
    pub dlfeat: Dlfeat,
}

impl TransmuteSafe for IdNmsp {}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Nsfeat {
    pub thinp: bool,
    pub nsabp: bool,
    pub dae: bool,
    pub uidreuse: bool,
    pub optperf: bool,
    #[skip]
    __rsvd: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Flbas {
    pub format_index: B4,
    pub extended_data_lba: bool,
    #[skip]
    __rsvd: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Mc {
    pub extended_data_lba: bool,
    pub separate_meta: bool,
    #[skip]
    __rsvd: B6,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Dpc {
    pub type_1: bool,
    pub type_2: bool,
    pub type_3: bool,
    pub header: bool,
    pub footer: bool,
    #[skip]
    __rsvd: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Nmic {
    pub shared_nmsp: bool,
    #[skip]
    __rsvd: B7,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Rescap {
    pub persist_through_pwr_loss: bool,
    pub write_exclusive: bool,
    pub exclusive_access: bool,
    pub write_exclusive_registrant_only: bool,
    pub exclusive_access_registrant_only: bool,
    pub write_exclusive_all_registrants: bool,
    pub exclusive_access_all_registrants: bool,
    pub ignore_existing_key: bool,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Fpi {
    pub percent_remaining: B7,
    pub supported: bool,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Dlfeat {
    pub dealloc_read_behavior: B3,
    pub supported: bool,
    pub guard_crc: bool,
    #[skip]
    __rsvd: B3,
}

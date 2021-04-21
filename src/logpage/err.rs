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

use crate::{Reserved, StatusField, TransmuteSafe};

use modular_bitfield::prelude::*;

#[test_structure(size = 64)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrLogEntry {
    #[loc(0:7)]
    pub err_count: u64,
    #[loc(8:9)]
    pub submission_queue_id: u16,
    #[loc(10:11)]
    pub cmd_id: u16,
    #[loc(12:13)]
    #[cfg_attr(feature = "serde", serde(with = "crate::status::StatusFieldUnpacked"))]
    pub status_field: StatusField,
    #[loc(14:15)]
    #[cfg_attr(feature = "serde", serde(with = "ParamErrLocUnpacked"))]
    pub param_err_loc: ParamErrLoc,
    #[loc(16:23)]
    pub lba: u64,
    #[loc(24:27)]
    pub nmsp: u32,
    #[loc(28:28)]
    pub vndr_specific_info_avail: u8,
    #[loc(29:29)]
    pub trtype: u8,
    #[loc(30:31)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd30: Reserved<2>,
    #[loc(32:39)]
    pub cmd_specific_info: u64,
    #[loc(40:41)]
    pub transport_type_specific_info: u16,
    #[loc(42:63)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd42: Reserved<22>,
}

impl TransmuteSafe for ErrLogEntry {}

#[bitfield]
#[derive(Clone, Copy)]
pub struct ParamErrLoc {
    pub byte: u8,
    pub bit: B3,
    #[skip]
    __: B5,
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "ParamErrLoc")]
struct ParamErrLocUnpacked {
    #[serde(getter = "ParamErrLoc::byte")]
    pub byte: u8,
    #[serde(getter = "ParamErrLoc::bit")]
    pub bit: u8,
}

#[cfg(feature = "serde")]
impl From<ParamErrLocUnpacked> for ParamErrLoc {
    fn from(unpacked: ParamErrLocUnpacked) -> Self {
        ParamErrLoc::new()
            .with_byte(unpacked.byte)
            .with_bit(unpacked.bit)
    }
}

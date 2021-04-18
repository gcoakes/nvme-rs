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

#[repr(packed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrLogEntry {
    pub err_count: u64,
    pub submission_queue_id: u16,
    pub cmd_id: u16,
    #[cfg_attr(feature = "serde", serde(with = "crate::status::StatusFieldUnpacked"))]
    pub status_field: StatusField,
    #[cfg_attr(feature = "serde", serde(with = "ParamErrLocUnpacked"))]
    pub param_err_loc: ParamErrLoc,
    pub lba: u64,
    pub nmsp: u32,
    pub vndr_specific_info_avail: u8,
    pub trtype: u8,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd30: Reserved<2>,
    pub cmd_specific_info: u64,
    pub transport_type_specific_info: u16,
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

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<ParamErrLoc>(), 2);
    assert_eq!(mem::size_of::<ErrLogEntry>(), 64);
}

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

use std::borrow::Cow;

use crate::{FixedStr, Reserved, TransmuteSafe};

use modular_bitfield::prelude::*;

#[repr(packed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FwSlotLog {
    #[cfg_attr(feature = "serde", serde(with = "ActiveFwInfoUnpacked"))]
    pub afi: ActiveFwInfo,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd1: Reserved<7>,
    pub frs: [FixedStr<8>; 7],
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd64: Reserved<448>,
}

impl TransmuteSafe for FwSlotLog {}

impl FwSlotLog {
    pub fn get_slot<'a>(&'a self, index: usize) -> Cow<'a, str> {
        self.frs[index].to_string_lossy()
    }
}

#[bitfield]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct ActiveFwInfo {
    pub active_slot: B3,
    #[skip]
    __: B1,
    pub next_active: B3,
    #[skip]
    __: B1,
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "ActiveFwInfo")]
struct ActiveFwInfoUnpacked {
    #[serde(getter = "ActiveFwInfo::active_slot")]
    pub active_slot: u8,
    #[serde(getter = "ActiveFwInfo::next_active")]
    pub next_active: u8,
}

#[cfg(feature = "serde")]
impl From<ActiveFwInfoUnpacked> for ActiveFwInfo {
    fn from(unpacked: ActiveFwInfoUnpacked) -> Self {
        ActiveFwInfo::new()
            .with_active_slot(unpacked.active_slot)
            .with_next_active(unpacked.next_active)
    }
}

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<FwSlotLog>(), 512);
}

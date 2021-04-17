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

use std::borrow::Cow;

use crate::FromBytes;

use modular_bitfield::prelude::*;

#[repr(packed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FwSlotLog {
    #[cfg_attr(feature = "serde", serde(with = "ActiveFwInfoUnpacked"))]
    pub afi: ActiveFwInfo,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd1: [u8; 7],
    #[cfg_attr(feature = "serde", serde(with = "frs_serde"))]
    pub frs: [[u8; 8]; 7],
    #[cfg_attr(feature = "serde", serde(skip, default = "rsvd64_def"))]
    __rsvd64: [u8; 448],
}

impl FwSlotLog {
    pub fn get_slot<'a>(&'a self, index: usize) -> Cow<'a, str> {
        String::from_utf8_lossy(&self.frs[index][..])
    }
}

impl FromBytes for FwSlotLog {
    fn from_bytes<'a>(bytes: &'a [u8]) -> &'a Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
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

#[cfg(feature = "serde")]
mod frs_serde {
    use std::convert::TryInto;

    use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(
        slots: &[[u8; 8]; 7],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(7))?;
        for slot in slots {
            seq.serialize_element(&String::from_utf8_lossy(&slot[..]))?;
        }
        seq.end()
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[[u8; 8]; 7], D::Error> {
        let mut slots: [String; 7] = Deserialize::deserialize(deserializer)?;
        for slot in slots.iter_mut() {
            while slot.len() < 8 {
                slot.push(' ');
            }
        }
        Ok([
            slots[0].as_bytes().try_into().unwrap(),
            slots[1].as_bytes().try_into().unwrap(),
            slots[2].as_bytes().try_into().unwrap(),
            slots[3].as_bytes().try_into().unwrap(),
            slots[4].as_bytes().try_into().unwrap(),
            slots[5].as_bytes().try_into().unwrap(),
            slots[6].as_bytes().try_into().unwrap(),
        ])
    }
}

fn rsvd64_def() -> [u8; 448] {
    [0; 448]
}

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<FwSlotLog>(), 512);
}

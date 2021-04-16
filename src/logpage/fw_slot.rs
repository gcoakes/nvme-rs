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

#[repr(packed)]
pub struct FwSlotLog {
    pub afi: u8,
    __rsvd1: [u8; 7],
    pub fw_rev_slots: [[u8; 8]; 7],
    __rsvd64: [u8; 448],
}

impl FwSlotLog {
    pub fn get_slot<'a>(&'a self, index: usize) -> Cow<'a, str> {
        String::from_utf8_lossy(&self.fw_rev_slots[index][..])
    }
}

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<FwSlotLog>(), 512);
}

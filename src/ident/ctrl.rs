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

use crate::FromBytes;

pub struct IdCtrl([u8; 4096]);

impl FromBytes for IdCtrl {
    fn from_bytes<'a>(bytes: &'a [u8]) -> &'a Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }
}

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<IdCtrl>(), 4096);
}

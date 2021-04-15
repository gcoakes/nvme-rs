/*
 * Copyright (c) 2021 Oakes, Gregory C. <gregcoakes@gmail.com>
 * Author: Oakes, Gregory C. <gregcoakes@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use crate::StatusField;

use modular_bitfield::prelude::*;

#[repr(packed)]
pub struct ErrLogEntry {
    pub err_count: u64,
    pub submission_queue_id: u16,
    pub command_id: u16,
    pub status_field: StatusField,
    pub param_err_loc: ParamErrLoc,
    pub lba: u64,
    pub nmsp: u32,
    pub vendor_specific_info_available: u8,
    pub trtype: u8,
    __rsvd30: u16,
    pub cmd_specific_info: u64,
    pub transport_type_specific_info: u16,
    __rsvd42: [u8; 22],
}

#[bitfield]
pub struct ParamErrLoc {
    byte: u8,
    bit: B3,
    #[skip]
    __: B5,
}

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<ParamErrLoc>(), 2);
    assert_eq!(mem::size_of::<ErrLogEntry>(), 64);
}

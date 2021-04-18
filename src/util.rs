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
    borrow::Cow,
    convert::{TryFrom, TryInto},
    mem::MaybeUninit,
};

pub trait FromBytes {
    fn from_bytes<'a>(bytes: &'a [u8]) -> Result<&'a Self, usize>;
}

/// Marker trait to indicate that struct can safely be transmuted or cast from
/// bytes without any undefined behavior. This means any contained enums fill
/// their entire repr space. Also, this most likely means the struct is packed.
pub trait TransmuteSafe {}

impl<T> FromBytes for T
where
    T: TransmuteSafe + Sized,
{
    fn from_bytes<'a>(bytes: &'a [u8]) -> Result<&'a Self, usize> {
        if bytes.len() == std::mem::size_of::<Self>() {
            Ok(unsafe { &*(bytes.as_ptr() as *const Self) })
        } else {
            Err(bytes.len())
        }
    }
}

impl<T> FromBytes for [T]
where
    T: TransmuteSafe + Sized,
{
    fn from_bytes<'a>(bytes: &'a [u8]) -> Result<&'a Self, usize> {
        if bytes.len() % std::mem::size_of::<T>() == 0 {
            Ok(unsafe {
                std::slice::from_raw_parts(
                    bytes.as_ptr() as *const T,
                    bytes.len() / std::mem::size_of::<T>(),
                )
            })
        } else {
            Err(bytes.len())
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Reserved<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Default for Reserved<SIZE> {
    fn default() -> Self {
        Reserved([0; SIZE])
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// TODO: Implement serde without copying.
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct FixedStr<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> FixedStr<SIZE> {
    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.0[..])
    }
}

impl<const SIZE: usize> From<FixedStr<SIZE>> for String {
    fn from(val: FixedStr<SIZE>) -> Self {
        val.to_string_lossy().to_string()
    }
}

impl<const SIZE: usize> TryFrom<&[u8]> for FixedStr<SIZE> {
    type Error = usize;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() > SIZE {
            Err(bytes.len())
        } else {
            Ok(FixedStr(unsafe {
                let mut cpy: [u8; SIZE] = MaybeUninit::uninit().assume_init();
                cpy[..].clone_from_slice(bytes);
                cpy
            }))
        }
    }
}

impl<const SIZE: usize> TryFrom<String> for FixedStr<SIZE> {
    type Error = usize;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

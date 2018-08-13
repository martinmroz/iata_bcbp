// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::ffi;
use std::ptr;
use std::str::FromStr;

use libc::{c_char, c_int};

use super::Bcbp;

/// Construct a new `Bcbp` by parsing the provided input string.
/// 
/// # Note
/// 
/// If the string passed in isn't a valid BCBP Type 'M' data string
/// this will return a null pointer.
///
/// # Safety
///
/// Make sure you destroy the object with [`bcbp_destroy()`] once you are
/// done with it.
///
/// [`BcbpDestroy()`]: fn.BcbpDestroy.html
#[no_mangle]
pub unsafe extern "C" fn BcbpCreateWithCString(input: *const c_char) -> *mut Bcbp {
    if input.is_null() {
        return ptr::null_mut();
    }

    let input_str = {
        if let Ok(value) = ffi::CStr::from_ptr(input).to_str() {
            value
        } else {
            return ptr::null_mut();
        }
    };

    if let Ok(bcbp) = Bcbp::from_str(input_str) {
        Box::into_raw(Box::new(bcbp))
    } else {
        ptr::null_mut()
    }
}

/// Destroy a `Bcbp` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn BcbpDestroy(bcbp: *mut Bcbp) {
    if !bcbp.is_null() {
        drop(Box::from_raw(bcbp));
    }
}

/// Destroy a string copied out of a `Bcbp` instance once copied.
#[no_mangle]
pub unsafe extern "C" fn BcbpDestroyString(string: *mut c_char) {
    if !string.is_null() {
        drop(ffi::CString::from_raw(string));
    }
}

/// Copies the name of a passenger from a boarding pass.
/// 
/// # Note
///
/// If the `Bcbp` object provided is null, this will return a null pointer.
///
/// # Safety
///
/// Make sure you destroy the result with [`BcbpDestroyString()`] once you are
/// done with it.
///
/// [`BcbpDestroyString()`]: fn.BcbpDestroyString.html
#[no_mangle]
pub unsafe extern "C" fn BcbpCopyPassengerName(bcbp: *mut Bcbp) -> *mut c_char {
    if bcbp.is_null() {
        return ptr::null_mut();
    }

    if let Ok(passenger_name) = ffi::CString::new((&*bcbp).passenger_name()) {
        passenger_name.into_raw()
    } else {
        ptr::null_mut()
    }
}

/// Copies the name of a passenger from a boarding pass.
/// 
/// # Note
///
/// If the `Bcbp` object provided is null, this will return 0.
#[no_mangle]
pub unsafe extern "C" fn BcbpGetNumberOfLegs(bcbp: *mut Bcbp) -> c_int {
    if bcbp.is_null() {
        0
    } else {
        (&*bcbp).legs().len() as c_int
    }
}

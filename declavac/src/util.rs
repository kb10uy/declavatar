use std::{ffi::c_char, slice::from_raw_parts, str::from_utf8};

#[macro_export]
macro_rules! as_ref {
    ($ptr:ident, &str, $len:ident) => {
        let $ptr = if $ptr.is_null() {
            return $crate::StatusCode::InvalidPointer;
        } else {
            let slice = unsafe { from_raw_parts($ptr as *const u8, $len as usize) };
            match from_utf8(slice) {
                Ok(s) => s,
                Err(_) => return $crate::StatusCode::Utf8Error,
            }
        };
    };

    ($ptr:ident, &$t:ty) => {
        let $ptr = if $ptr.is_null() {
            return $crate::StatusCode::InvalidPointer;
        } else {
            unsafe { &*$ptr as &$t }
        };
    };

    ($ptr:ident, &mut $t:ty) => {
        let $ptr = if $ptr.is_null() {
            return $crate::StatusCode::InvalidPointer;
        } else {
            unsafe { &mut *$ptr as &mut $t }
        };
    };

    ($ptr:ident, box $t:ty) => {
        let $ptr = if $ptr.is_null() {
            return $crate::StatusCode::InvalidPointer;
        } else {
            unsafe { Box::from_raw($ptr as *mut $t) }
        };
    };

    (mut $ptr:ident, box $t:ty) => {
        let mut $ptr = if $ptr.is_null() {
            return $crate::StatusCode::InvalidPointer;
        } else {
            unsafe { Box::from_raw($ptr as *mut $t) }
        };
    };
}

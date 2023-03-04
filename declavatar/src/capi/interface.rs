use crate::capi::interop::{Declavatar, StatusCode};

use std::ffi::{c_char, CStr};

macro_rules! as_ref {
    ($ptr:ident, &str) => {
        let $ptr = if $ptr.is_null() {
            return StatusCode::InvalidPointer;
        } else {
            match unsafe { CStr::from_ptr($ptr).to_str() } {
                Ok(s) => s,
                Err(_) => return StatusCode::Utf8Error,
            }
        };
    };

    ($ptr:ident, &$t:ty) => {
        let $ptr = if $ptr.is_null() {
            return StatusCode::InvalidPointer;
        } else {
            unsafe { &*$ptr as &$t }
        };
    };

    ($ptr:ident, &mut $t:ty) => {
        let $ptr = if $ptr.is_null() {
            return StatusCode::InvalidPointer;
        } else {
            unsafe { &mut *$ptr as &mut $t }
        };
    };

    ($ptr:ident, box $t:ty) => {
        let $ptr = if $ptr.is_null() {
            return StatusCode::InvalidPointer;
        } else {
            unsafe { Box::from_raw($ptr as *mut $t) }
        };
    };

    (mut $ptr:ident, box $t:ty) => {
        let mut $ptr = if $ptr.is_null() {
            return StatusCode::InvalidPointer;
        } else {
            unsafe { Box::from_raw($ptr as *mut $t) }
        };
    };
}

#[no_mangle]
pub extern "system" fn DeclavatarInit() -> *mut Declavatar {
    let boxed = Box::new(Declavatar::new());
    Box::into_raw(boxed) as *mut Declavatar
}

#[no_mangle]
pub extern "system" fn DeclavatarFree(da: *mut Declavatar) -> StatusCode {
    as_ref!(da, box Declavatar);

    drop(da);

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarReset(da: *mut Declavatar) -> StatusCode {
    as_ref!(da, &mut Declavatar);

    da.reset();

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarCompile(da: *mut Declavatar, source: *const c_char) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(source, &str);

    match da.compile(source) {
        Ok(()) => StatusCode::Success,
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "system" fn DeclavatarGetAvatarJson(
    da: *mut Declavatar,
    avatar_json: *mut *const c_char,
    avatar_json_len: *mut u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(avatar_json, &mut *const c_char);
    as_ref!(avatar_json_len, &mut u32);

    match da.avatar_json() {
        Ok(json) => {
            *avatar_json = json.as_ptr() as *const i8;
            *avatar_json_len = json.len() as u32;
            StatusCode::Success
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "system" fn DeclavatarGetErrorsCount(
    da: *mut Declavatar,
    errors: *mut u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(errors, &mut u32);

    *errors = da.errors().len() as u32;

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarGetError(
    da: *mut Declavatar,
    index: u32,
    error_kind: *mut u32,
    error_str: *mut *const c_char,
    error_len: *mut u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(error_kind, &mut u32);
    as_ref!(error_str, &mut *const c_char);
    as_ref!(error_len, &mut u32);

    let index = index as usize;
    let errors = da.errors();
    let (kind, message) = if index < errors.len() {
        &errors[index]
    } else {
        return StatusCode::InvalidPointer;
    };

    *error_kind = *kind as u32;
    *error_str = message.as_ptr() as *const i8;
    *error_len = message.len() as u32;

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarPushExampleErrors(da: *mut Declavatar) -> StatusCode {
    as_ref!(da, &mut Declavatar);

    da.push_example_errors();

    StatusCode::Success
}

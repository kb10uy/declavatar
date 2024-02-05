mod serialization;
mod state;
mod util;

use crate::state::{CompiledState, DeclavatarState};

use std::ffi::c_char;

/// Declavatar status code.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclavatarStatus {
    /// Operation succeeded.
    Success = 0,

    /// Given string has invalid UTF-8 sequence.
    Utf8Error = 1,

    /// Internal JSON serialization error.
    JsonError = 2,

    /// Error occurred in compilation.
    CompileError = 3,

    /// Given pointer was invalid.
    InvalidPointer = 128,
}

/// Initializes declavatar compiler state.
#[no_mangle]
pub extern "C" fn declavatar_init() -> *mut DeclavatarState {
    let boxed = Box::new(DeclavatarState::new());
    Box::into_raw(boxed)
}

/// Frees declavatar compiler state.
#[no_mangle]
pub extern "C" fn declavatar_free(da: *mut DeclavatarState) -> DeclavatarStatus {
    as_ref!(da, box DeclavatarState);
    drop(da);
    DeclavatarStatus::Success
}

/// Clears defined symbols/localizations/arbittach definitions.
///
/// # Safety
/// Given pointer `da` must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_clear(da: *mut DeclavatarState) -> DeclavatarStatus {
    as_ref!(da, &mut DeclavatarState);

    da.arguments_mut().clear();
    DeclavatarStatus::Success
}

/// Clears defined symbols/localizations/arbittach definitions.
///
/// # Safety
/// Given pointers must be valid.
/// `path` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_add_library_path(
    da: *mut DeclavatarState,
    path: *const c_char,
    path_len: u32,
) -> DeclavatarStatus {
    as_ref!(da, &mut DeclavatarState);
    as_ref!(path, &str, path_len);

    let args = da.arguments_mut();
    args.add_library_path(path);

    DeclavatarStatus::Success
}

/// Defines a symbol for given state.
///
/// # Safety
/// Given pointers must be valid.
/// `symbol` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_define_symbol(
    da: *mut DeclavatarState,
    symbol: *const c_char,
    symbol_len: u32,
) -> DeclavatarStatus {
    as_ref!(da, &mut DeclavatarState);
    as_ref!(symbol, &str, symbol_len);

    let args = da.arguments_mut();
    args.define_symbol(symbol);

    DeclavatarStatus::Success
}

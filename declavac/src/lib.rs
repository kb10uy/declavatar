mod serialization;
mod state;
mod util;

use declavatar::decl_v2::DeclarationFormat;

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

/// Declavatar definition file format..
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclavatarFormat {
    /// S-expression.
    Sexpr = 1,

    /// Lua.
    Lua = 2,
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

/// Defines a localization for given state.
///
/// # Safety
/// Given pointers must be valid.
/// `key`, `value` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_define_localization(
    da: *mut DeclavatarState,
    key: *const c_char,
    key_len: u32,
    value: *const c_char,
    value_len: u32,
) -> DeclavatarStatus {
    as_ref!(da, &mut DeclavatarState);
    as_ref!(key, &str, key_len);
    as_ref!(value, &str, value_len);

    let args = da.arguments_mut();
    args.define_localization(key, value);

    DeclavatarStatus::Success
}

/// Registers Arbitrary Attachment (arbittach) definition.
///
/// # Safety
/// Given pointers must be valid.
/// `definition` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_register_arbittach(
    da: *mut DeclavatarState,
    definition: *const c_char,
    definition_len: u32,
) -> DeclavatarStatus {
    as_ref!(da, &mut DeclavatarState);
    as_ref!(definition, &str, definition_len);

    DeclavatarStatus::Success
}

/// Compiles definition with format.
///
/// # Safety
/// Given pointers must be valid.
/// `source` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_compile(
    da: *mut DeclavatarState,
    compiled_state: *mut *mut CompiledState,
    source: *const c_char,
    source_len: u32,
    format_kind: DeclavatarFormat,
) -> DeclavatarStatus {
    as_ref!(da, &mut DeclavatarState);
    as_ref!(compiled_state, &mut *mut CompiledState);
    as_ref!(source, &str, source_len);

    #[allow(unreachable_patterns)]
    let format = match format_kind {
        DeclavatarFormat::Sexpr => DeclarationFormat::Sexpr,
        DeclavatarFormat::Lua => DeclarationFormat::Lua,
        _ => return DeclavatarStatus::InvalidPointer,
    };
    match da.compile(source, format) {
        Ok(compiled) => {
            *compiled_state = Box::into_raw(Box::new(compiled));
        }
        Err(_) => {
            return DeclavatarStatus::JsonError;
        }
    }

    DeclavatarStatus::Success
}

/// Frees compiled result.
///
/// # Safety
/// Given pointer must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_free_compiled(
    compiled_state: *mut CompiledState,
) -> DeclavatarStatus {
    as_ref!(compiled_state, box DeclavatarState);
    drop(compiled_state);
    DeclavatarStatus::Success
}

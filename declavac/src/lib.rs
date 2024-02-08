mod serialization;
mod state;
mod util;

use crate::state::{CompiledState, DeclavatarState};

use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

use declavatar::{decl_v2::DeclarationFormat, i18n::get_log_messages};

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

    /// Given value was invalid.
    InvalidValue = 129,
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

/// Fetches compile log localization.
///
/// # Safety
/// Given pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_log_localization(
    locale: *const c_char,
    locale_len: u32,
    json_string: *mut *const c_char,
    json_string_len: *mut u32,
) -> DeclavatarStatus {
    as_ref!(locale, &str, locale_len);
    as_ref!(json_string, &mut *const c_char);
    as_ref!(json_string_len, &mut u32);

    let Some(loge_l10n_json) = get_log_messages(locale) else {
        return DeclavatarStatus::InvalidValue;
    };
    *json_string = loge_l10n_json.as_ptr() as *const i8;
    *json_string_len = loge_l10n_json.len() as u32;

    DeclavatarStatus::Success
}

/// Initializes declavatar compiler state.
#[no_mangle]
pub extern "C" fn declavatar_init() -> *mut c_void {
    let boxed = Box::new(DeclavatarState::new());
    Box::into_raw(boxed) as *mut c_void
}

/// Frees declavatar compiler state.
#[no_mangle]
pub extern "C" fn declavatar_free(declavatar_state: *mut c_void) -> DeclavatarStatus {
    as_ref!(declavatar_state, box DeclavatarState);
    drop(declavatar_state);
    DeclavatarStatus::Success
}

/// Fetches last error message.
///
/// # Safety
/// Given pointer `da` must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_last_error(
    declavatar_state: *mut c_void,
    message: *mut *const c_char,
    message_len: *mut u32,
) -> DeclavatarStatus {
    as_ref!(declavatar_state, &mut DeclavatarState);

    let (message_str, status) = declavatar_state.last_error();
    if let Some(message_str) = message_str {
        *message = message_str.as_ptr() as *const i8;
        *message_len = message_str.len() as u32;
    } else {
        *message = null();
        *message_len = 0;
    }
    status
}

/// Clears defined symbols/localizations/arbittach definitions.
///
/// # Safety
/// Given pointer `da` must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_clear(declavatar_state: *mut c_void) -> DeclavatarStatus {
    as_ref!(declavatar_state, &mut DeclavatarState);

    declavatar_state.clear()
}

/// Clears defined symbols/localizations/arbittach definitions.
///
/// # Safety
/// Given pointers must be valid.
/// `path` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_add_library_path(
    declavatar_state: *mut c_void,
    path: *const c_char,
    path_len: u32,
) -> DeclavatarStatus {
    as_ref!(declavatar_state, &mut DeclavatarState);
    as_ref!(path, &str, path_len);

    declavatar_state.add_library_path(path)
}

/// Defines a symbol for given state.
///
/// # Safety
/// Given pointers must be valid.
/// `symbol` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_define_symbol(
    declavatar_state: *mut c_void,
    symbol: *const c_char,
    symbol_len: u32,
) -> DeclavatarStatus {
    as_ref!(declavatar_state, &mut DeclavatarState);
    as_ref!(symbol, &str, symbol_len);

    declavatar_state.define_symbol(symbol)
}

/// Defines a localization for given state.
///
/// # Safety
/// Given pointers must be valid.
/// `key`, `value` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_define_localization(
    declavatar_state: *mut c_void,
    key: *const c_char,
    key_len: u32,
    value: *const c_char,
    value_len: u32,
) -> DeclavatarStatus {
    as_ref!(declavatar_state, &mut DeclavatarState);
    as_ref!(key, &str, key_len);
    as_ref!(value, &str, value_len);

    declavatar_state.define_localization(key, value)
}

/// Registers Arbitrary Attachment (arbittach) definition.
///
/// # Safety
/// Given pointers must be valid.
/// `definition` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_register_arbittach(
    declavatar_state: *mut c_void,
    definition: *const c_char,
    definition_len: u32,
) -> DeclavatarStatus {
    as_ref!(declavatar_state, &mut DeclavatarState);
    as_ref!(definition, &str, definition_len);

    declavatar_state.add_attachment(definition)
}

/// Compiles definition with format.
///
/// # Safety
/// Given pointers must be valid.
/// `source` does not have to NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_compile(
    declavatar_state: *const c_void,
    compiled_state: *mut *mut c_void,
    source: *const c_char,
    source_len: u32,
    format_kind: DeclavatarFormat,
) -> DeclavatarStatus {
    as_ref!(declavatar_state, &DeclavatarState);
    as_ref!(compiled_state, &mut *mut CompiledState);
    as_ref!(source, &str, source_len);

    #[allow(unreachable_patterns)]
    let format = match format_kind {
        DeclavatarFormat::Sexpr => DeclarationFormat::Sexpr,
        DeclavatarFormat::Lua => DeclarationFormat::Lua,
        _ => return DeclavatarStatus::InvalidValue,
    };

    let (compiled, status) = declavatar_state.compile(source, format);
    *compiled_state = Box::into_raw(Box::new(compiled));
    status
}

/// Frees compiled result.
///
/// # Safety
/// Given pointer must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_compiled_free(compiled_state: *mut c_void) -> DeclavatarStatus {
    as_ref!(compiled_state, box CompiledState);
    drop(compiled_state);
    DeclavatarStatus::Success
}

/// Retrieves the pointer of compiled JSON string.
///
/// # Safety
/// Given pointer must be valid.
/// Returned string is not NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_compiled_avatar_json(
    compiled_state: *const c_void,
    json_string: *mut *const c_char,
    json_string_len: *mut u32,
) -> DeclavatarStatus {
    as_ref!(compiled_state, &CompiledState);
    as_ref!(json_string, &mut *const c_char);
    as_ref!(json_string_len, &mut u32);

    if let Some(json_str) = compiled_state.avatar_json() {
        *json_string = json_str.as_ptr() as *const i8;
        *json_string_len = json_str.len() as u32;
    } else {
        *json_string = null();
        *json_string_len = 0;
    }

    DeclavatarStatus::Success
}

/// Retrieves the count of compile logs.
///
/// # Safety
/// Given pointer must be valid.
#[no_mangle]
pub unsafe extern "C" fn declavatar_compiled_logs_count(
    compiled_state: *const c_void,
    logs_count: *mut u32,
) -> DeclavatarStatus {
    as_ref!(compiled_state, &CompiledState);
    as_ref!(logs_count, &mut u32);

    *logs_count = compiled_state.logs_len() as u32;

    DeclavatarStatus::Success
}

/// Retrieves the pointer of compile log as JSON.
///
/// # Safety
/// Given pointer must be valid.
/// Returned string is not NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn declavatar_compiled_log(
    compiled_state: *const c_void,
    index: u32,
    json_string: *mut *const c_char,
    json_string_len: *mut u32,
) -> DeclavatarStatus {
    as_ref!(compiled_state, &CompiledState);
    as_ref!(json_string, &mut *const c_char);
    as_ref!(json_string_len, &mut u32);

    let Some(json_str) = compiled_state.log_json(index as usize) else {
        return DeclavatarStatus::JsonError;
    };
    *json_string = json_str.as_ptr() as *const i8;
    *json_string_len = json_str.len() as u32;

    DeclavatarStatus::Success
}

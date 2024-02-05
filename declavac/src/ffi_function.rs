#[no_mangle]
pub extern "system" fn DeclavatarInit() -> *mut Declavatar {
    let boxed = Box::new(Declavatar::new());
    Box::into_raw(boxed)
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
pub extern "system" fn DeclavatarAddLibraryPath(
    da: *mut Declavatar,
    path: *const c_char,
    path_len: u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(path, &str, path_len);

    da.add_library_path(path);

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarDefineSymbol(
    da: *mut Declavatar,
    symbol: *const c_char,
    symbol_len: u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(symbol, &str, symbol_len);

    da.define_symbol(symbol);

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarDefineLocalization(
    da: *mut Declavatar,
    key: *const c_char,
    key_len: u32,
    value: *const c_char,
    value_len: u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(key, &str, key_len);
    as_ref!(value, &str, value_len);

    da.define_localization(key, value);

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarCompile(
    da: *mut Declavatar,
    source: *const c_char,
    source_len: u32,
    format_kind: u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(source, &str, source_len);

    match da.compile(source, format_kind) {
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
pub extern "system" fn DeclavatarGetLogsCount(da: *mut Declavatar, errors: *mut u32) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(errors, &mut u32);

    *errors = da.log_jsons().len() as u32;

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarGetLogJson(
    da: *mut Declavatar,
    index: u32,
    error_str: *mut *const c_char,
    error_len: *mut u32,
) -> StatusCode {
    as_ref!(da, &mut Declavatar);
    as_ref!(error_str, &mut *const c_char);
    as_ref!(error_len, &mut u32);

    let index = index as usize;
    let errors = da.log_jsons();
    let log_json = if index < errors.len() {
        &errors[index]
    } else {
        return StatusCode::InvalidPointer;
    };

    *error_str = log_json.as_ptr() as *const i8;
    *error_len = log_json.len() as u32;

    StatusCode::Success
}

#[no_mangle]
pub extern "system" fn DeclavatarGetI18n(
    i18n_key: *const c_char,
    i18n_key_len: u32,
    i18n_json: *mut *const c_char,
    i18n_len: *mut u32,
) -> StatusCode {
    as_ref!(i18n_key, &str, i18n_key_len);
    as_ref!(i18n_json, &mut *const c_char);
    as_ref!(i18n_len, &mut u32);

    let json = if let Some(log_locale) = i18n_key.strip_prefix("log.") {
        match get_log_messages(log_locale) {
            Some(j) => j,
            None => return StatusCode::InvalidPointer,
        }
    } else {
        return StatusCode::InvalidPointer;
    };

    *i18n_json = json.as_ptr() as *const i8;
    *i18n_len = json.len() as u32;

    StatusCode::Success
}

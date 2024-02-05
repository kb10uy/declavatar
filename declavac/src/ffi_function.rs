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

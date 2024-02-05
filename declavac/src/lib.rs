mod serialization;
mod state;
mod util;

use crate::state::{CompiledState, DeclavatarState};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Success = 0,
    Utf8Error = 1,
    CompileError = 2,
    AlreadyInUse = 3,
    NotCompiled = 4,
    InvalidPointer = 128,
}

#[no_mangle]
pub extern "system" fn declavatar_init() -> *mut DeclavatarState {
    let boxed = Box::new(DeclavatarState::new());
    Box::into_raw(boxed)
}

#[no_mangle]
pub extern "system" fn declavatar_free(da: *mut DeclavatarState) -> StatusCode {
    as_ref!(da, box DeclavatarState);

    drop(da);

    StatusCode::Success
}

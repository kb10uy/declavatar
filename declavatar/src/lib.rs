pub mod avatar;
pub mod decl;

pub mod avatar_v2;
pub mod decl_v2;

#[cfg(feature = "cdylib")]
mod capi;

#[cfg(test)]
mod testing;

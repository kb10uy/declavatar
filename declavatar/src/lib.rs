pub mod avatar;
pub mod decl;
pub mod decl_v2;

#[cfg(feature = "cdylib")]
mod capi;

#[cfg(test)]
mod testing;

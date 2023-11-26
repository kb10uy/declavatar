pub mod avatar;
pub mod decl;

#[cfg(feature = "cdylib")]
mod capi;

#[cfg(test)]
mod testing;

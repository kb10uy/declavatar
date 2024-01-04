pub mod asset;
pub mod avatar;
pub mod controller;
pub mod driver;
pub mod export;
pub mod layer;
pub mod menu;
pub mod parameter;

pub trait StaticTypeName {
    const TYPE_NAME: &'static str;
}

#[macro_export]
macro_rules! static_type_name_impl {
    ($t:ident) => {
        impl $crate::decl_v2::data::StaticTypeName for $t {
            const TYPE_NAME: &'static str = stringify!($t);
        }

        impl $crate::decl_v2::data::StaticTypeName for &$t {
            const TYPE_NAME: &'static str = stringify!($t);
        }
    };
}

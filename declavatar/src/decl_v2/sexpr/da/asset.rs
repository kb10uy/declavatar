use crate::decl_v2::{
    data::asset::{DeclAsset, DeclAssets},
    sexpr::{argument::SeparateArguments, error::KetosResult, register_function, KetosValueExt},
};

use ketos::{Arity, Name, NameStore, Scope, Value};

pub fn register_asset_function(scope: &Scope) {
    register_function(scope, "assets", declare_assets, Arity::Min(0), Some(&[]));
    register_function(scope, "material", declare_material, Arity::Exact(1), Some(&[]));
    register_function(scope, "animation", declare_animation, Arity::Exact(1), Some(&[]));
}

fn declare_assets(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let mut assets = vec![];
    for asset_value in args.args_after_recursive(function_name, 0)? {
        assets.push(asset_value.downcast_foreign_ref::<&DeclAsset>().cloned()?);
    }
    Ok(DeclAssets { assets }.into())
}

fn declare_material(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let key: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAsset::Material(key.to_string()).into())
}

fn declare_animation(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let key: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAsset::Animation(key.to_string()).into())
}

#[cfg(test)]
mod test {
    use crate::decl_v2::{
        data::asset::{DeclAsset, DeclAssets},
        sexpr::test::eval_da_value,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn reads_assets() {
        assert_eq!(
            eval_da_value::<DeclAssets>(r#"(da/assets)"#),
            DeclAssets { assets: vec![] }
        );
        assert_eq!(
            eval_da_value::<DeclAssets>(r#"(da/assets (da/material "hoge"))"#),
            DeclAssets {
                assets: vec![DeclAsset::Material("hoge".to_string())]
            }
        );
        assert_eq!(
            eval_da_value::<DeclAssets>(r#"(da/assets (list (da/material "hoge") (da/animation "fuga")))"#),
            DeclAssets {
                assets: vec![
                    DeclAsset::Material("hoge".to_string()),
                    DeclAsset::Animation("fuga".to_string())
                ]
            }
        );
    }

    #[test]
    fn reads_material() {
        assert_eq!(
            eval_da_value::<DeclAsset>(r#"(da/material "hoge")"#),
            DeclAsset::Material("hoge".to_string())
        );
    }

    #[test]
    fn reads_animation() {
        assert_eq!(
            eval_da_value::<DeclAsset>(r#"(da/animation "hoge")"#),
            DeclAsset::Animation("hoge".to_string())
        );
    }
}

use crate::decl_v2::{
    data::export::{DeclExport, DeclExports},
    sexpr::{argument::SeparateArguments, error::KetosResult, register_function, KetosValueExt},
};

use ketos::{Arity, Name, NameStore, Scope, Value};

pub fn register_export_function(scope: &Scope) {
    register_function(scope, "exports", declare_exports, Arity::Min(0), Some(&[]));
    register_function(scope, "gate", declare_gate, Arity::Exact(1), Some(&[]));
    register_function(scope, "guard", declare_guard, Arity::Exact(2), Some(&[]));
}

fn declare_exports(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let mut exports = vec![];
    for export_value in args.args_after_recursive(function_name, 0)? {
        exports.push(export_value.downcast_foreign_ref::<&DeclExport>().cloned()?);
    }
    Ok(DeclExports { exports }.into())
}

fn declare_gate(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclExport::Gate(name.to_string()).into())
}

fn declare_guard(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let gate_name: &str = args.exact_arg(function_name, 0)?;
    let parameter: &str = args.exact_arg(function_name, 1)?;
    Ok(DeclExport::Guard(gate_name.to_string(), parameter.to_string()).into())
}

#[cfg(test)]
mod test {
    use crate::decl_v2::{
        data::export::{DeclExport, DeclExports},
        sexpr::test::eval_da_value,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn reads_exports() {
        assert_eq!(
            eval_da_value::<DeclExports>(r#"(da/exports)"#),
            DeclExports { exports: vec![] }
        );
        assert_eq!(
            eval_da_value::<DeclExports>(r#"(da/exports (da/gate "hoge"))"#),
            DeclExports {
                exports: vec![DeclExport::Gate("hoge".to_string())]
            }
        );
        assert_eq!(
            eval_da_value::<DeclExports>(r#"(da/exports (list (da/gate "hoge") (da/guard "fuga" "piyo")))"#),
            DeclExports {
                exports: vec![
                    DeclExport::Gate("hoge".to_string()),
                    DeclExport::Guard("fuga".to_string(), "piyo".to_string()),
                ]
            }
        );
    }

    #[test]
    fn reads_gate() {
        assert_eq!(
            eval_da_value::<DeclExport>(r#"(da/gate "hoge")"#),
            DeclExport::Gate("hoge".to_string())
        );
    }

    #[test]
    fn reads_guard() {
        assert_eq!(
            eval_da_value::<DeclExport>(r#"(da/guard "hoge" "fuga")"#),
            DeclExport::Guard("hoge".to_string(), "fuga".to_string())
        );
    }
}

use crate::decl_v2::{
    data::attachment::DeclAttachmentValue,
    sexpr::{argument::SeparateArguments, error::KetosResult, register_function},
};

use ketos::{Arity, Name, NameStore, Scope, Value};

pub fn register_value_function(scope: &Scope) {
    register_function(
        scope,
        "vector2",
        |n, f, a| value_vector(n, f, a, 2),
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "vector3",
        |n, f, a| value_vector(n, f, a, 3),
        Arity::Exact(3),
        Some(&[]),
    );
    register_function(
        scope,
        "vector4",
        |n, f, a| value_vector(n, f, a, 4),
        Arity::Exact(4),
        Some(&[]),
    );
    register_function(scope, "game-object", value_game_object, Arity::Exact(1), Some(&[]));
    register_function(scope, "material", value_material, Arity::Exact(1), Some(&[]));
    register_function(
        scope,
        "animation-clip",
        value_animation_clip,
        Arity::Exact(1),
        Some(&[]),
    );
}

fn value_vector(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
    length: usize,
) -> KetosResult<Value> {
    let values = (0..length)
        .map(|i| args.exact_arg(function_name, i))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(DeclAttachmentValue::Vector(values).into())
}

fn value_game_object(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAttachmentValue::GameObject(name.to_string()).into())
}

fn value_material(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAttachmentValue::Material(name.to_string()).into())
}

fn value_animation_clip(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAttachmentValue::AnimationClip(name.to_string()).into())
}

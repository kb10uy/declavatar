use crate::{
    avatar::{
        data::{Parameter, ParameterScope, ParameterType},
        transformer::{failure, success, Compiled, Context, LogKind},
    },
    decl::data::{
        Parameter as DeclParameter, ParameterScope as DeclParameterScope,
        ParameterType as DeclParameterType, Parameters as DeclParameters,
    },
};

pub fn compile_parameter_blocks(
    ctx: &mut Context,
    parameters_blocks: Vec<DeclParameters>,
) -> Compiled<Vec<Parameter>> {
    let mut parameters: Vec<Parameter> = vec![];

    let decl_parameters = parameters_blocks.into_iter().flat_map(|pb| pb.parameters);
    for decl_parameter in decl_parameters {
        let Some(parameter) = compile_parameter(ctx, decl_parameter, &parameters) else {
            continue;
        };

        parameters.push(parameter);
    }

    success(parameters)
}

fn compile_parameter(
    ctx: &mut Context,
    decl_parameter: DeclParameter,
    declared: &[Parameter],
) -> Compiled<Parameter> {
    let name = decl_parameter.name.clone();
    let value_type = match decl_parameter.ty {
        DeclParameterType::Int(dv) => ParameterType::Int(dv.unwrap_or(0)),
        DeclParameterType::Float(dv) => ParameterType::Float(dv.unwrap_or(0.0)),
        DeclParameterType::Bool(dv) => ParameterType::Bool(dv.unwrap_or(false)),
    };
    let scope = match (decl_parameter.scope, decl_parameter.save) {
        (Some(DeclParameterScope::Internal), None | Some(false)) => ParameterScope::Internal,
        (Some(DeclParameterScope::Local), None) => ParameterScope::Local(false),
        (Some(DeclParameterScope::Local), Some(saved)) => ParameterScope::Local(saved),
        (None | Some(DeclParameterScope::Synced), None) => ParameterScope::Synced(false),
        (None | Some(DeclParameterScope::Synced), Some(saved)) => ParameterScope::Synced(saved),

        (Some(DeclParameterScope::Internal), Some(true)) => {
            ctx.log_error(LogKind::InternalMustBeTransient(decl_parameter.name));
            return failure();
        }
    };

    if let Some(defined) = declared.iter().find(|p| p.name == decl_parameter.name) {
        if defined.value_type != value_type || defined.scope != scope {
            ctx.log_error(LogKind::IncompatibleParameterDeclaration(
                decl_parameter.name,
            ));
        }
        return failure();
    }

    success(Parameter {
        name,
        value_type,
        scope,
    })
}

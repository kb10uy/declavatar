use crate::{
    compiler::{Compile, Compiler},
    decl::{
        compiler::{deconstruct_node, DeclCompiler},
        data::{Parameter, ParameterScope, ParameterType, Parameters},
        error::{DeclError, DeclErrorKind, Result},
    },
};

use kdl::KdlNode;

pub const NODE_NAME_PARAMETERS: &str = "parameters";
pub const NODE_NAME_INT: &str = "int";
pub const NODE_NAME_FLOAT: &str = "float";
pub const NODE_NAME_BOOL: &str = "bool";
pub const SCOPE_INTERNAL: &str = "internal";
pub const SCOPE_LOCAL: &str = "local";
pub const SCOPE_SYNCED: &str = "synced";

pub(super) struct ForParameters;
impl Compile<(ForParameters, &KdlNode)> for DeclCompiler {
    type Output = Parameters;

    fn compile(&mut self, (_, node): (ForParameters, &KdlNode)) -> Result<Parameters> {
        let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_PARAMETERS), Some(true))?;

        let mut parameters = vec![];
        for child in children {
            let child_name = child.name().value();
            let parameter = match child_name {
                NODE_NAME_INT | NODE_NAME_FLOAT | NODE_NAME_BOOL => {
                    self.parse((ForParameter, child))?
                }
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            };
            parameters.push(parameter);
        }

        Ok(Parameters { parameters })
    }
}

struct ForParameter;
impl Compile<(ForParameter, &KdlNode)> for DeclCompiler {
    type Output = Parameter;

    fn compile(&mut self, (_, node): (ForParameter, &KdlNode)) -> Result<Parameter> {
        let (name, entries, _) = deconstruct_node(node, None, Some(false))?;

        let parameter_name = entries.get_argument(0, "name")?;
        let save = entries.try_get_property("save")?;
        let scope = match entries.try_get_property::<&str>("scope")? {
            Some(SCOPE_INTERNAL) => Some(ParameterScope::Internal),
            Some(SCOPE_LOCAL) => Some(ParameterScope::Local),
            Some(SCOPE_SYNCED) => Some(ParameterScope::Synced),
            None => None,

            Some(_) => {
                return Err(DeclError::new(
                    node.span(),
                    DeclErrorKind::InvalidNodeDetected,
                ))
            }
        };
        let ty = match name {
            NODE_NAME_INT => {
                let default = entries.try_get_property("default")?.map(|x: i64| x as u8);
                ParameterType::Int(default)
            }
            NODE_NAME_FLOAT => {
                let default = entries.try_get_property("default")?;
                ParameterType::Float(default)
            }
            NODE_NAME_BOOL => {
                let default = entries.try_get_property("default")?;
                ParameterType::Bool(default)
            }
            _ => unreachable!("parameter type already refined here"),
        };

        Ok(Parameter {
            ty,
            save,
            scope,
            name: parameter_name,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        compiler::Compile,
        decl::{
            data::{Parameter, ParameterScope, ParameterType},
            DeclCompiler,
        },
        testing::parse_node,
    };

    use super::ForParameters;

    #[test]
    fn parameters_block_compiles() {
        let block_doc = parse_node(
            r#"
            parameters {
                int "foo"
                float "bar" save=false
                bool "baz" scope="local"
            }
            "#,
        );
        let block_node = &block_doc.nodes()[0];

        let mut compiler = DeclCompiler::new();
        let block = compiler
            .compile((ForParameters, block_node))
            .expect("failed to compile parameters block");
        assert_eq!(block.parameters.len(), 3);
        assert_eq!(
            block.parameters[0],
            Parameter {
                name: "foo".to_string(),
                save: None,
                scope: None,
                ty: ParameterType::Int(None)
            }
        );
        assert_eq!(
            block.parameters[1],
            Parameter {
                name: "bar".to_string(),
                save: Some(false),
                scope: None,
                ty: ParameterType::Float(None)
            }
        );
        assert_eq!(
            block.parameters[2],
            Parameter {
                name: "baz".to_string(),
                save: None,
                scope: Some(ParameterScope::Local),
                ty: ParameterType::Bool(None)
            }
        );
    }

    #[test]
    fn parameters_block_fails_compile() {
        let invalid_doc1 = parse_node(
            r#"
            parameters {
                unknown "hoge"
            }
            "#,
        );
        let invalid_node1 = &invalid_doc1.nodes()[0];

        let mut compiler = DeclCompiler::new();
        compiler
            .compile((ForParameters, invalid_node1))
            .expect_err("compiles parameters block incorrectly");
    }
}

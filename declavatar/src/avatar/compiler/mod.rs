mod animations;
mod drivers;
mod menu;
mod parameters;

use crate::{
    avatar::{
        data::{Avatar, Parameter, ParameterScope, ParameterType},
        error::{AvatarError, Result},
    },
    compiler::{Compile, Compiler, ErrorStackCompiler, Validate},
    decl::data::Avatar as DeclAvatar,
};

pub type AvatarCompiler = ErrorStackCompiler<AvatarError>;

impl Compile<DeclAvatar> for AvatarCompiler {
    type Output = Option<Avatar>;

    fn compile(&mut self, avatar: DeclAvatar) -> Result<Option<Avatar>> {
        let name = {
            let decl_name = avatar.name.trim();
            if decl_name.is_empty() {
                self.error("invalid avatar name".into());
                return Ok(None);
            }
            decl_name.to_string()
        };

        let parameters = self.parse(avatar.parameters_blocks)?;
        let animation_groups = self.parse((avatar.animations_blocks, &parameters))?;
        let driver_groups = self.parse((avatar.drivers_blocks, &parameters, &animation_groups))?;
        let top_menu_group = self.parse((avatar.menu_blocks, &parameters, &animation_groups))?;
        Ok(Some(Avatar {
            name,
            parameters,
            animation_groups,
            driver_groups,
            top_menu_group,
        }))
    }
}

impl Validate<(&Vec<Parameter>, &str, &ParameterType, bool)> for AvatarCompiler {
    fn validate(
        &mut self,
        (parameters, name, ty, should_exposed): (&Vec<Parameter>, &str, &ParameterType, bool),
    ) -> Result<bool> {
        let parameter = match parameters.iter().find(|p| p.name == name) {
            Some(p) => p,
            None => {
                self.error(format!("parameter '{name}' not found"));
                return Ok(false);
            }
        };
        if parameter.scope == ParameterScope::Internal && should_exposed {
            self.error(format!("parameter '{name}' must not internal"));
            return Ok(false);
        }
        match (&parameter.value_type, ty) {
            (ParameterType::Int(_), ParameterType::Int(_)) => Ok(true),
            (ParameterType::Float(_), ParameterType::Float(_)) => Ok(true),
            (ParameterType::Bool(_), ParameterType::Bool(_)) => Ok(true),
            _ => {
                self.error(format!(
                    "parameter '{}' has wrong type; {} expected",
                    name,
                    ty.type_name()
                ));
                Ok(false)
            }
        }
    }
}

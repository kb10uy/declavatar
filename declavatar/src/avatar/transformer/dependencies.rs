use crate::avatar::{
    data::{
        AnimationGroup, AnimationGroupContent, Asset, AssetType, GroupOption, Parameter,
        ParameterScope, ParameterType,
    },
    transformer::{failure, success, Compiled, Context, LogKind},
};

pub struct CompiledSources {
    parameters: Vec<Parameter>,
    assets: Vec<Asset>,
}

impl CompiledSources {
    pub fn new(parameters: Vec<Parameter>, assets: Vec<Asset>) -> CompiledSources {
        CompiledSources { parameters, assets }
    }

    pub fn find_parameter(
        &self,
        ctx: &mut Context,
        name: &str,
        ty: ParameterType,
        scope: ParameterScope,
    ) -> Compiled<&Parameter> {
        let parameter = self.find_parameter_untyped(ctx, name, scope)?;
        if !parameter.value_type.matches(ty) {
            ctx.log_error(LogKind::ParameterTypeRequirement(name.to_string(), ty));
            return failure();
        }
        success(parameter)
    }

    pub fn find_parameter_untyped(
        &self,
        ctx: &mut Context,
        name: &str,
        scope: ParameterScope,
    ) -> Compiled<&Parameter> {
        let parameter = match self.parameters.iter().find(|p| p.name == name) {
            Some(p) => p,
            None => {
                ctx.log_error(LogKind::ParameterNotFound(name.to_string()));
                return failure();
            }
        };
        if !parameter.scope.suitable_for(scope) {
            ctx.log_error(LogKind::ParameterScopeRequirement(name.to_string(), scope));
            return failure();
        }
        success(parameter)
    }

    pub fn find_asset(&self, ctx: &mut Context, name: &str, ty: AssetType) -> Compiled<&Asset> {
        let asset = match self.assets.iter().find(|p| p.key == name) {
            Some(p) => p,
            None => {
                ctx.log_error(LogKind::AssetNotFound(name.to_string()));
                return failure();
            }
        };
        if asset.asset_type != ty {
            ctx.log_error(LogKind::AssetTypeRequirement(name.to_string(), ty));
            return failure();
        }
        success(asset)
    }
}

pub struct CompiledAnimations {
    sources: CompiledSources,
    animations: Vec<AnimationGroup>,
}

impl CompiledAnimations {
    pub fn new(
        dependencies: CompiledSources,
        animations: Vec<AnimationGroup>,
    ) -> CompiledAnimations {
        CompiledAnimations {
            sources: dependencies,
            animations,
        }
    }

    pub fn sources(&self) -> &CompiledSources {
        &self.sources
    }

    pub fn find_animation_group(&self, ctx: &mut Context, name: &str) -> Compiled<&AnimationGroup> {
        if let Some(ag) = self.animations.iter().find(|a| a.name == name) {
            success(ag)
        } else {
            ctx.log_error(LogKind::AnimationGroupNotFound(name.to_string()));
            failure()
        }
    }

    pub fn deconstruct(self) -> (Vec<Parameter>, Vec<Asset>, Vec<AnimationGroup>) {
        (
            self.sources.parameters,
            self.sources.assets,
            self.animations,
        )
    }
}

pub trait AnimationGroupFilterExt {
    fn ensure_group(&self, ctx: &mut Context) -> Compiled<(&str, &[GroupOption])>;
    fn ensure_switch(&self, ctx: &mut Context) -> Compiled<&str>;
}

impl AnimationGroupFilterExt for AnimationGroup {
    fn ensure_group(&self, ctx: &mut Context) -> Compiled<(&str, &[GroupOption])> {
        if let AnimationGroup {
            content:
                AnimationGroupContent::Group {
                    parameter, options, ..
                },
            ..
        } = self
        {
            success((parameter, options))
        } else {
            ctx.log_error(LogKind::AnimationGroupMustBeGroup(self.name.to_string()));
            failure()
        }
    }

    fn ensure_switch(&self, ctx: &mut Context) -> Compiled<&str> {
        if let AnimationGroup {
            content: AnimationGroupContent::Switch { parameter, .. },
            ..
        } = self
        {
            success(parameter)
        } else {
            ctx.log_error(LogKind::AnimationGroupMustBeSwitch(self.name.to_string()));
            failure()
        }
    }
}

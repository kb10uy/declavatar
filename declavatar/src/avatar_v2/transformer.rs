pub mod asset;
pub mod avatar;
pub mod controller;
pub mod driver;
pub mod layer;
pub mod menu;
pub mod parameter;

use crate::avatar_v2::{
    data::{
        asset::{Asset, AssetType},
        layer::{Layer, LayerContent, LayerGroupOption},
        parameter::{Parameter, ParameterScope, ParameterType},
    },
    logger::{LogKind, Logger},
};

pub(super) use self::avatar::compile_avatar;

// Renamed for future change
type Compiled<T> = Option<T>;

// Reserved as a function for future change
#[inline]
fn success<T>(t: T) -> Compiled<T> {
    Some(t)
}

// Reserved as a function for future change
#[inline]
fn failure<T>() -> Compiled<T> {
    None
}

pub struct CompiledSources<'a> {
    parameters: &'a [Parameter],
    assets: &'a [Asset],
}

impl<'c, 'a> CompiledSources<'a> {
    pub fn new(parameters: &'a [Parameter], assets: &'a [Asset]) -> CompiledSources<'a> {
        CompiledSources { parameters, assets }
    }

    pub fn find_parameter(
        &'a self,
        ctx: &'c mut Logger,
        name: &'a str,
        ty: ParameterType,
        scope: ParameterScope,
    ) -> Compiled<&'a Parameter> {
        let parameter = self.find_parameter_untyped(ctx, name, scope)?;
        if !parameter.value_type.matches(ty) {
            ctx.log_error(LogKind::ParameterTypeRequirement(
                name.to_string(),
                ty.type_name().to_string(),
            ));
            return failure();
        }
        success(parameter)
    }

    pub fn find_parameter_untyped(
        &'a self,
        ctx: &'c mut Logger,
        name: &'a str,
        scope: ParameterScope,
    ) -> Compiled<&'a Parameter> {
        let parameter = match self.parameters.iter().find(|p| p.name == name) {
            Some(p) => p,
            None => {
                ctx.log_error(LogKind::ParameterNotFound(name.to_string()));
                return failure();
            }
        };
        if !parameter.scope.suitable_for(scope) {
            ctx.log_error(LogKind::ParameterScopeRequirement(
                name.to_string(),
                scope.name().to_string(),
            ));
            return failure();
        }
        success(parameter)
    }

    pub fn find_asset(
        &'a self,
        ctx: &'a mut Logger,
        name: &'a str,
        ty: AssetType,
    ) -> Compiled<&'a Asset> {
        let asset = match self.assets.iter().find(|p| p.key == name) {
            Some(p) => p,
            None => {
                ctx.log_error(LogKind::AssetNotFound(name.to_string()));
                return failure();
            }
        };
        if asset.asset_type != ty {
            ctx.log_error(LogKind::AssetTypeRequirement(
                name.to_string(),
                ty.type_name().to_string(),
            ));
            return failure();
        }
        success(asset)
    }
}

pub struct CompiledAnimations<'a> {
    sources: CompiledSources<'a>,
    layers: Vec<&'a Layer>,
}

impl<'c, 'a: 'c> CompiledAnimations<'a> {
    pub fn new(sources: CompiledSources<'a>, layers: Vec<&'a Layer>) -> CompiledAnimations<'a> {
        CompiledAnimations { sources, layers }
    }

    pub fn sources(&'a self) -> &'a CompiledSources {
        &self.sources
    }

    pub fn find_group(
        &'a self,
        ctx: &'c mut Logger,
        name: &'a str,
    ) -> Compiled<(&'a str, &'a [LayerGroupOption])> {
        let layer = self.find_layer(ctx, name)?;
        if let Layer {
            content: LayerContent::Group {
                parameter, options, ..
            },
            ..
        } = layer
        {
            success((parameter, options))
        } else {
            ctx.log_error(LogKind::AnimationGroupMustBeGroup(name.to_string()));
            failure()
        }
    }

    pub fn find_switch(&'a self, ctx: &'c mut Logger, name: &'a str) -> Compiled<&'a str> {
        let layer = self.find_layer(ctx, name)?;
        if let Layer {
            content: LayerContent::Switch { parameter, .. },
            ..
        } = layer
        {
            success(parameter)
        } else {
            ctx.log_error(LogKind::AnimationGroupMustBeSwitch(name.to_string()));
            failure()
        }
    }

    pub fn find_puppet(&'a self, ctx: &'c mut Logger, name: &'a str) -> Compiled<&'a str> {
        let layer = self.find_layer(ctx, name)?;
        if let Layer {
            content: LayerContent::Puppet { parameter, .. },
            ..
        } = layer
        {
            success(parameter)
        } else {
            ctx.log_error(LogKind::AnimationGroupMustBePuppet(name.to_string()));
            failure()
        }
    }

    fn find_layer(&'a self, ctx: &'c mut Logger, name: &'a str) -> Compiled<&'a Layer> {
        if let Some(ag) = self.layers.iter().find(|a| a.name == name) {
            success(ag)
        } else {
            ctx.log_error(LogKind::AnimationGroupNotFound(name.to_string()));
            failure()
        }
    }
}

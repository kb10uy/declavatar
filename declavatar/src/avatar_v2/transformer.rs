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
        parameter::{Parameter, ParameterScope, ParameterType},
    },
    logger::{Log, Logger},
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

#[derive(Debug, Clone)]
pub struct DeclaredLayer {
    pub name: String,
    pub layer_type: DeclaredLayerType,
}

#[derive(Debug, Clone)]
pub enum DeclaredLayerType {
    Group(String, Vec<(String, usize)>),
    Switch(String),
    Puppet(String),
    Raw,
}

pub struct FirstPassData {
    parameters: Vec<Parameter>,
    assets: Vec<Asset>,
    layers: Vec<DeclaredLayer>,
}

impl FirstPassData {
    pub fn new(
        parameters: Vec<Parameter>,
        assets: Vec<Asset>,
        layers: Vec<DeclaredLayer>,
    ) -> FirstPassData {
        FirstPassData {
            parameters,
            assets,
            layers,
        }
    }

    pub fn take_back(self) -> (Vec<Parameter>, Vec<Asset>) {
        (self.parameters, self.assets)
    }

    pub fn find_parameter(
        &self,
        logger: &Logger,
        name: &str,
        ty: ParameterType,
        scope: ParameterScope,
    ) -> Compiled<&Parameter> {
        let parameter = self.find_parameter_untyped(logger, name, scope)?;
        if !parameter.value_type.matches(ty) {
            logger.log(Log::ParameterTypeRequirement(
                name.to_string(),
                ty.type_name().to_string(),
            ));
            return failure();
        }
        success(parameter)
    }

    pub fn find_parameter_untyped(
        &self,
        logger: &Logger,
        name: &str,
        scope: ParameterScope,
    ) -> Compiled<&Parameter> {
        let parameter = match self.parameters.iter().find(|p| p.name == name) {
            Some(p) => p,
            None => {
                logger.log(Log::ParameterNotFound(name.to_string()));
                return failure();
            }
        };
        if !parameter.scope.suitable_for(scope) {
            logger.log(Log::ParameterScopeRequirement(
                name.to_string(),
                scope.name().to_string(),
            ));
            return failure();
        }
        success(parameter)
    }

    pub fn find_asset(&self, logger: &Logger, name: &str, ty: AssetType) -> Compiled<&Asset> {
        let asset = match self.assets.iter().find(|p| p.key == name) {
            Some(p) => p,
            None => {
                logger.log(Log::AssetNotFound(name.to_string()));
                return failure();
            }
        };
        if asset.asset_type != ty {
            logger.log(Log::AssetTypeRequirement(
                name.to_string(),
                ty.type_name().to_string(),
            ));
            return failure();
        }
        success(asset)
    }

    pub fn find_group(&self, logger: &Logger, name: &str) -> Compiled<(&str, &[(String, usize)])> {
        // don't check parameter in first pass
        let layer = self.find_layer(logger, name)?;
        if let DeclaredLayerType::Group(parameter, options) = layer {
            success((&parameter, &options))
        } else {
            logger.log(Log::LayerMustBeGroup(name.to_string()));
            failure()
        }
    }

    pub fn find_switch(&self, logger: &Logger, name: &str) -> Compiled<&str> {
        // don't check parameter in first pass
        let layer = self.find_layer(logger, name)?;
        if let DeclaredLayerType::Switch(parameter) = layer {
            success(parameter)
        } else {
            logger.log(Log::LayerMustBeSwitch(name.to_string()));
            failure()
        }
    }

    pub fn find_puppet(&self, logger: &Logger, name: &str) -> Compiled<&str> {
        // don't check parameter in first pass
        let layer = self.find_layer(logger, name)?;
        if let DeclaredLayerType::Puppet(parameter) = layer {
            success(parameter)
        } else {
            logger.log(Log::LayerMustBePuppet(name.to_string()));
            failure()
        }
    }

    fn find_layer(&self, logger: &Logger, name: &str) -> Compiled<&DeclaredLayerType> {
        if let Some(dl) = self.layers.iter().find(|a| a.name == name) {
            success(&dl.layer_type)
        } else {
            logger.log(Log::LayerNotFound(name.to_string()));
            failure()
        }
    }
}

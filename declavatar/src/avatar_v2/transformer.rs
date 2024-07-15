pub mod asset;
pub mod attachment;
pub mod avatar;
pub mod controller;
pub mod driver;
pub mod export;
pub mod layer;
pub mod menu;
pub mod parameter;

use crate::{
    avatar_v2::{
        data::{
            asset::{Asset, AssetType},
            parameter::{Parameter, ParameterQuery, ParameterType, QualifiedParameter},
        },
        log::Log,
    },
    log::Logger,
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DeclaredLayerType {
    Group(ParameterQuery, Vec<(String, usize)>),
    Switch(ParameterQuery),
    Puppet(ParameterQuery),
    SwitchGate(String),
    Raw(Vec<String>),
}

pub struct FirstPassData {
    parameters: Vec<Parameter>,
    assets: Vec<Asset>,
    exported_gates: Vec<String>,
    layers: Vec<DeclaredLayer>,
}

impl FirstPassData {
    pub fn new(
        parameters: Vec<Parameter>,
        assets: Vec<Asset>,
        exported_gates: Vec<String>,
        layers: Vec<DeclaredLayer>,
    ) -> FirstPassData {
        FirstPassData {
            parameters,
            assets,
            exported_gates,
            layers,
        }
    }

    pub fn take_back(self) -> (Vec<Parameter>, Vec<Asset>) {
        (self.parameters, self.assets)
    }

    pub fn find_untyped_parameter(&self, logger: &Logger<Log>, query: &ParameterQuery) -> Compiled<QualifiedParameter> {
        let parameter = match self.parameters.iter().find_map(|p| query.qualify_match(p)) {
            Some(p) => p,
            None => {
                logger.log(Log::ParameterNotFound(query.querying_name().to_string()));
                return failure();
            }
        };
        success(parameter)
    }

    pub fn find_read_parameter(
        &self,
        logger: &Logger<Log>,
        query: &ParameterQuery,
        ty: ParameterType,
    ) -> Compiled<QualifiedParameter> {
        let qualified = self.find_untyped_parameter(logger, query)?;
        if !qualified.value_type.matches(ty) {
            logger.log(Log::ParameterTypeRequirement(
                qualified.name,
                ty.type_name().to_string(),
            ));
            return failure();
        }
        success(qualified)
    }

    pub fn find_writable_parameter(
        &self,
        logger: &Logger<Log>,
        query: &ParameterQuery,
        ty: ParameterType,
    ) -> Compiled<QualifiedParameter> {
        if !matches!(query, ParameterQuery::Declared(_)) {
            logger.log(Log::ParameterNotWritable(query.querying_name().to_string()));
            return failure();
        }
        let name = self.find_read_parameter(logger, query, ty)?;
        success(name)
    }

    pub fn find_asset(&self, logger: &Logger<Log>, name: &str, ty: AssetType) -> Compiled<&Asset> {
        let asset = match self.assets.iter().find(|p| p.key == name) {
            Some(p) => p,
            None => {
                logger.log(Log::AssetNotFound(name.to_string()));
                return failure();
            }
        };
        if asset.asset_type != ty {
            logger.log(Log::AssetTypeRequirement(name.to_string(), ty.type_name().to_string()));
            return failure();
        }
        success(asset)
    }

    pub fn find_group(&self, logger: &Logger<Log>, name: &str) -> Compiled<(&ParameterQuery, &[(String, usize)])> {
        let layer = self.find_layer(logger, name)?;
        let DeclaredLayerType::Group(query, options) = layer else {
            logger.log(Log::LayerMustBeGroup(name.to_string()));
            return failure();
        };
        success((query, options))
    }

    pub fn find_switch(&self, logger: &Logger<Log>, name: &str) -> Compiled<&ParameterQuery> {
        let layer = self.find_layer(logger, name)?;
        let DeclaredLayerType::Switch(query) = layer else {
            logger.log(Log::LayerMustBeSwitch(name.to_string()));
            return failure();
        };
        success(query)
    }

    pub fn find_puppet(&self, logger: &Logger<Log>, name: &str) -> Compiled<&ParameterQuery> {
        let layer = self.find_layer(logger, name)?;
        let DeclaredLayerType::Puppet(query) = layer else {
            logger.log(Log::LayerMustBePuppet(name.to_string()));
            return failure();
        };
        success(query)
    }

    pub fn find_raw(&self, logger: &Logger<Log>, name: &str) -> Compiled<&[String]> {
        let layer = self.find_layer(logger, name)?;
        let DeclaredLayerType::Raw(state_names) = layer else {
            logger.log(Log::LayerMustBeRaw(name.to_string()));
            return failure();
        };
        success(state_names)
    }

    fn find_gate(&self, logger: &Logger<Log>, name: &str) -> Compiled<&str> {
        if let Some(gate) = self.exported_gates.iter().find(|a| a == &name) {
            success(gate)
        } else {
            logger.log(Log::GateNotFound(name.to_string()));
            failure()
        }
    }

    fn find_layer(&self, logger: &Logger<Log>, name: &str) -> Compiled<&DeclaredLayerType> {
        if let Some(dl) = self.layers.iter().find(|a| a.name == name) {
            success(&dl.layer_type)
        } else {
            logger.log(Log::LayerNotFound(name.to_string()));
            failure()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnsetValue {
    Active,
    Inactive,
}

impl UnsetValue {
    pub const fn as_bool(self) -> bool {
        match self {
            UnsetValue::Active => true,
            UnsetValue::Inactive => false,
        }
    }

    pub const fn as_f64(self) -> f64 {
        match self {
            UnsetValue::Active => 1.0,
            UnsetValue::Inactive => 0.0,
        }
    }

    pub fn replace_f64(self, base: Option<f64>) -> f64 {
        base.unwrap_or(self.as_f64())
    }

    pub fn replace_bool(self, base: Option<bool>) -> bool {
        base.unwrap_or(self.as_bool())
    }
}

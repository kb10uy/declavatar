use declavatar_derive::EnumLog;

#[derive(Debug, Clone, EnumLog)]
pub enum Log {
    #[log_error("avatar.invalid_name")]
    InvalidAvatarName(String),

    #[log_error("parameter.internal_must_transient")]
    InternalMustBeTransient(String),
    #[log_error("parameter.incompatible_declaration")]
    IncompatibleParameterDeclaration(String),
    #[log_error("parameter.not_found")]
    ParameterNotFound(String),
    #[log_error("parameter.type_requirement")]
    ParameterTypeRequirement(String, String),
    #[log_error("parameter.scope_requirement")]
    ParameterScopeRequirement(String, String),

    #[log_error("asset.incompatible_declaration")]
    IncompatibleAssetDeclaration(String),
    #[log_error("asset.not_found")]
    AssetNotFound(String),
    #[log_error("asset.type_requirement")]
    AssetTypeRequirement(String, String),

    #[log_error("gate.not_found")]
    GateNotFound(String),

    #[log_error("layer.not_found")]
    LayerNotFound(String),
    #[log_error("layer.duplicate_name")]
    DuplicateLayerName(String),
    #[log_error("layer.must_group")]
    LayerMustBeGroup(String),
    #[log_error("layer.must_switch")]
    LayerMustBeSwitch(String),
    #[log_error("layer.must_puppet")]
    LayerMustBePuppet(String),
    #[log_error("layer.must_raw")]
    LayerMustBeRaw(String),
    #[log_error("layer.group.invalid_copy")]
    LayerGroupInvalidCopy,
    #[log_error("layer.switch.indeterminate")]
    LayerSwitchIndeterminateSource,
    #[log_error("layer.puppet.cant_drive")]
    LayerPuppetCannotDrive,
    #[log_error("layer.puppet.must_inlined")]
    LayerPuppetOptionMustBeInlined,
    #[log_error("layer.puppet.out_of_range")]
    LayerKeyframeOutOfRange(f64),
    #[log_error("layer.raw.state_not_found")]
    LayerStateNotFound(String),
    #[log_error("layer.raw.invalid_condition")]
    LayerInvalidCondition,
    #[log_error("layer.option.not_found")]
    LayerOptionNotFound(String),
    #[log_error("layer.option_must_exclusive")]
    LayerOptionMustBeExclusive,

    #[log_error("menu.invalid_drive")]
    MenuInvalidDrive,

    #[log_error("driver.shape_indeterminate_change")]
    LayerIndeterminateShapeChange(String),
    #[log_error("driver.material_indeterminate_change")]
    LayerIndeterminateMaterialChange(usize),
}

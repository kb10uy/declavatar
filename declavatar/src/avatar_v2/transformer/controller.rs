use crate::{
    avatar_v2::{
        data::layer::Layer,
        log::Log,
        transformer::{
            layer::{
                compile_group_layer, compile_puppet_layer, compile_raw_layer, compile_switch_layer,
                first_pass_group_layer, first_pass_puppet_layer, first_pass_raw_layer, first_pass_switch_layer,
            },
            success, Compiled, DeclaredLayer, FirstPassData,
        },
    },
    decl_v2::data::{controller::DeclFxController, layer::DeclControllerLayer},
    log::Logger,
};

use std::collections::HashSet;

pub fn first_pass_fx_controller_blocks(
    logger: &Logger<Log>,
    fx_controller_blocks: &[DeclFxController],
) -> Compiled<Vec<DeclaredLayer>> {
    let mut declared_layers = vec![];
    for decl_fx_controller in fx_controller_blocks {
        for decl_layer in &decl_fx_controller.layers {
            let declared_layer = match decl_layer {
                DeclControllerLayer::Group(decl_group_layer) => first_pass_group_layer(logger, decl_group_layer),
                DeclControllerLayer::Switch(decl_switch_layer) => first_pass_switch_layer(logger, decl_switch_layer),
                DeclControllerLayer::Puppet(decl_puppet_layer) => first_pass_puppet_layer(logger, decl_puppet_layer),
                DeclControllerLayer::Raw(decl_raw_layer) => first_pass_raw_layer(logger, decl_raw_layer),
            };
            let Some(declared_layer) = declared_layer else {
                continue;
            };
            declared_layers.push(declared_layer);
        }
    }
    success(declared_layers)
}

pub fn compile_fx_controller_blocks(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    fx_controller_blocks: Vec<DeclFxController>,
) -> Compiled<Vec<Layer>> {
    let mut layers = vec![];
    let mut used_group_names: HashSet<String> = HashSet::new();
    for (index, decl_fx_controller) in fx_controller_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("fx-controller {index}"));
        for decl_layer in decl_fx_controller.layers {
            let layer = match decl_layer {
                DeclControllerLayer::Group(decl_group_layer) => {
                    compile_group_layer(&logger, first_pass, decl_group_layer)
                }
                DeclControllerLayer::Switch(decl_switch_layer) => {
                    compile_switch_layer(&logger, first_pass, decl_switch_layer)
                }
                DeclControllerLayer::Puppet(decl_puppet_layer) => {
                    compile_puppet_layer(&logger, first_pass, decl_puppet_layer)
                }
                DeclControllerLayer::Raw(decl_raw_layer) => compile_raw_layer(&logger, first_pass, decl_raw_layer),
            };
            let Some(layer) = layer else {
                continue;
            };

            if used_group_names.contains(&layer.name) {
                logger.log(Log::DuplicateLayerName(layer.name.clone()));
            } else {
                used_group_names.insert(layer.name.clone());
            }

            layers.push(layer);
        }
    }

    success(layers)
}

use crate::{
    avatar_v2::{
        data::layer::Layer,
        logger::{Log, Logger},
        transformer::{
            layer::{
                compile_group_layer, compile_puppet_layer, compile_raw_layer, compile_switch_layer,
            },
            success, Compiled, CompiledSources,
        },
    },
    decl_v2::data::{controller::DeclFxController, layer::DeclControllerLayer},
};

use std::collections::HashSet;

pub fn compile_fx_controller_blocks(
    logger: &mut Logger,
    sources: &CompiledSources,
    fx_controller_blocks: Vec<DeclFxController>,
) -> Compiled<Vec<Layer>> {
    let mut layers = vec![];

    let mut used_group_names: HashSet<String> = HashSet::new();
    let decl_fx_controllers = fx_controller_blocks.into_iter().flat_map(|fx| fx.layers);
    for decl_layer in decl_fx_controllers {
        let layer = match decl_layer {
            DeclControllerLayer::Group(decl_group_layer) => {
                compile_group_layer(logger, sources, decl_group_layer)
            }
            DeclControllerLayer::Switch(decl_switch_layer) => {
                compile_switch_layer(logger, sources, decl_switch_layer)
            }
            DeclControllerLayer::Puppet(decl_puppet_layer) => {
                compile_puppet_layer(logger, sources, decl_puppet_layer)
            }
            DeclControllerLayer::Raw(decl_raw_layer) => {
                compile_raw_layer(logger, sources, decl_raw_layer)
            }
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

    success(layers)
}

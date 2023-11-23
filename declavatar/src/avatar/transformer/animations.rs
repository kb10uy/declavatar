use crate::{
    avatar::{
        data::{AnimationGroup, GroupOption},
        transformer::{
            dependencies::CompiledSources, failure, success, Compiled, Context, LogKind,
        },
    },
    decl::data::{
        AnimationElement as DeclAnimationElement, AnimationGroup as DeclAnimationGroup,
        AnimationSwitch as DeclAnimationSwitch, Animations as DeclAnimations,
        GroupBlock as DeclGroupBlock, Layer as DeclLayer, Puppet as DeclPuppet,
    },
};

use std::collections::HashSet;

pub fn compile_animations_blocks(
    ctx: &mut Context,
    sources: &CompiledSources,
    animations_blocks: Vec<DeclAnimations>,
) -> Compiled<Vec<AnimationGroup>> {
    let mut animation_groups = vec![];

    let mut used_group_names: HashSet<String> = HashSet::new();
    let decl_animations = animations_blocks.into_iter().flat_map(|ab| ab.elements);
    for decl_animation in decl_animations {
        let animation_group = match decl_animation {
            DeclAnimationElement::Group(group) => compile_group(ctx, sources, group),
            DeclAnimationElement::Switch(switch) => compile_switch(ctx, sources, switch),
            DeclAnimationElement::Puppet(puppet) => compile_puppet(ctx, sources, puppet),
            DeclAnimationElement::Layer(layer) => compile_raw_layer(ctx, sources, layer),
        };
        let Some(animation_group) = animation_group else {
            continue;
        };

        if used_group_names.contains(&animation_group.name) {
            ctx.log_warn(LogKind::DuplicateGroupName(animation_group.name.clone()));
        } else {
            used_group_names.insert(animation_group.name.clone());
        }

        animation_groups.push(animation_group);
    }

    success(animation_groups)
}

fn compile_group(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_group: DeclAnimationGroup,
) -> Compiled<AnimationGroup> {
    failure()
}

fn compile_group_option(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_group_block: DeclGroupBlock,
) -> Compiled<GroupOption> {
    failure()
}

fn compile_switch(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_switch: DeclAnimationSwitch,
) -> Compiled<AnimationGroup> {
    failure()
}

fn compile_puppet(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_puppet: DeclPuppet,
) -> Compiled<AnimationGroup> {
    failure()
}

fn compile_raw_layer(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_layer: DeclLayer,
) -> Compiled<AnimationGroup> {
    failure()
}

use crate::{
    compiler::Compile,
    decl::{
        compiler::{deconstruct_node, DeclCompiler},
        data::{
            AnimationElement, AnimationGroup, AnimationSwitch, Animations, DriveTarget, GroupBlock,
            Preventions, Puppet, PuppetKeyframe, Target,
        },
        error::{DeclError, DeclErrorKind, Result},
    },
};

use kdl::{KdlNode, KdlValue};

pub const NODE_NAME_ANIMATIONS: &str = "animations";
const NODE_NAME_GROUP: &str = "group";
const NODE_NAME_SWITCH: &str = "switch";
const NODE_NAME_PUPPET: &str = "puppet";
const NODE_NAME_PARAMETER: &str = "parameter";
const NODE_NAME_MESH: &str = "mesh";
const NODE_NAME_PREVENT: &str = "prevent";
const NODE_NAME_DEFAULT: &str = "default";
const NODE_NAME_OPTION: &str = "option";
const NODE_NAME_SHAPE: &str = "shape";
const NODE_NAME_OBJECT: &str = "object";
const NODE_NAME_MATERIAL: &str = "material";
const NODE_NAME_KEYFRAME: &str = "keyframe";

pub(super) struct ForAnimations;
impl Compile<(ForAnimations, &KdlNode)> for DeclCompiler {
    type Output = Animations;

    fn compile(&mut self, (_, node): (ForAnimations, &KdlNode)) -> Result<Animations> {
        let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_ANIMATIONS), Some(true))?;

        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_GROUP => AnimationElement::Group(self.compile((ForGroup, child))?),
                NODE_NAME_SWITCH => AnimationElement::Switch(self.compile((ForSwitch, child))?),
                NODE_NAME_PUPPET => AnimationElement::Puppet(self.compile((ForPuppet, child))?),
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::MustHaveChildren,
                    ));
                }
            };
            elements.push(element);
        }

        Ok(Animations { elements })
    }
}

struct ForGroup;
impl Compile<(ForGroup, &KdlNode)> for DeclCompiler {
    type Output = AnimationGroup;

    fn compile(&mut self, (_, node): (ForGroup, &KdlNode)) -> Result<AnimationGroup> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_GROUP), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut default_mesh = None;
        let mut parameter = None;
        let mut preventions = Preventions::default();
        let mut default_block = None;
        let mut options = vec![];

        let mut option_order = 1;
        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, None)?;

            match child_name {
                NODE_NAME_MESH => {
                    default_mesh = child_entries.try_get_argument(0)?;
                }
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_PREVENT => {
                    preventions.mouth = preventions
                        .mouth
                        .or(child_entries.try_get_property("mouth")?);
                    preventions.eyelids = preventions
                        .eyelids
                        .or(child_entries.try_get_property("eyelids")?);
                }
                NODE_NAME_DEFAULT => {
                    if default_block.is_some() {
                        return Err(DeclError::new(
                            child.name().span(),
                            DeclErrorKind::DuplicateNodeFound,
                        ));
                    }
                    default_block = Some(self.compile((ForGroupBlock, child, 0))?);
                }
                NODE_NAME_OPTION => {
                    options.push(self.compile((ForGroupBlock, child, option_order))?);
                    option_order += 1;
                }
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(AnimationGroup {
            name,
            default_mesh,
            parameter,
            preventions,
            default_block,
            options,
        })
    }
}

struct ForGroupBlock;
impl Compile<(ForGroupBlock, &KdlNode, usize)> for DeclCompiler {
    type Output = GroupBlock;

    fn compile(
        &mut self,
        (_, node, order): (ForGroupBlock, &KdlNode, usize),
    ) -> Result<GroupBlock> {
        let (name, entries, children) = deconstruct_node(node, None, None)?;
        let indeterminate = children.is_empty();

        let mut targets = vec![];
        let block_name;
        if indeterminate {
            // indeterminate option
            if name != NODE_NAME_OPTION {
                return Err(DeclError::new(
                    node.name().span(),
                    DeclErrorKind::InvalidNodeDetected,
                ));
            }

            let label: String = entries.get_argument(0, "label")?;
            let mesh = entries.try_get_property("mesh")?;
            let object = entries.try_get_property("object")?;
            let shape = entries.try_get_property("shape")?;
            let value = match entries.try_get_property::<&KdlValue>("value")? {
                Some(v) => {
                    if let Some(value) = v.as_f64() {
                        Some(DriveTarget::FloatParameter {
                            name: String::new(),
                            value,
                        })
                    } else if let Some(value) = v.as_bool() {
                        Some(DriveTarget::BoolParameter {
                            name: String::new(),
                            value,
                        })
                    } else {
                        return Err(DeclError::new(
                            node.name().span(),
                            DeclErrorKind::IncorrectType("float or bool"),
                        ));
                    }
                }
                None => None,
            };

            block_name = Some(label.clone());
            targets.push(Target::Indeterminate {
                label,
                object,
                mesh,
                shape,
                value,
            });
        } else {
            // determinate option
            block_name = match name {
                NODE_NAME_OPTION => Some(entries.get_argument(0, "name")?),
                NODE_NAME_DEFAULT => None,
                _ => unreachable!("block type already refined here"),
            };

            for child in children {
                let (child_name, child_entries, _) = deconstruct_node(child, None, Some(false))?;
                let target = match child_name {
                    NODE_NAME_SHAPE => {
                        let shape = child_entries.get_argument(0, "shape")?;
                        let mesh = child_entries.try_get_property("mesh")?;
                        let value = child_entries.try_get_property("value")?;
                        Target::Shape { shape, mesh, value }
                    }
                    NODE_NAME_OBJECT => {
                        let object = child_entries.get_argument(0, "object")?;
                        let value = child_entries.try_get_property("value")?;
                        Target::Object { object, value }
                    }
                    NODE_NAME_MATERIAL => {
                        let slot: i64 = child_entries.get_argument(0, "slot")?;
                        let mesh = child_entries.try_get_property("mesh")?;
                        let value = child_entries.try_get_property("value")?;
                        Target::Material {
                            slot: slot as usize,
                            value,
                            mesh,
                        }
                    }
                    _ => {
                        return Err(DeclError::new(
                            child.name().span(),
                            DeclErrorKind::InvalidNodeDetected,
                        ));
                    }
                };
                targets.push(target);
            }
        }

        Ok(GroupBlock {
            name: block_name,
            declared_order: order,
            indeterminate,
            targets,
        })
    }
}

struct ForSwitch;
impl Compile<(ForSwitch, &KdlNode)> for DeclCompiler {
    type Output = AnimationSwitch;

    fn compile(&mut self, (_, node): (ForSwitch, &KdlNode)) -> Result<AnimationSwitch> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_SWITCH), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut default_mesh = None;
        let mut parameter = None;
        let mut preventions = Preventions::default();
        let mut enabled = vec![];
        let mut disabled = vec![];

        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, Some(false))?;

            match child_name {
                NODE_NAME_MESH => {
                    default_mesh = child_entries.try_get_argument(0)?;
                }
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_PREVENT => {
                    preventions.mouth = preventions
                        .mouth
                        .or(child_entries.try_get_property("mouth")?);
                    preventions.eyelids = preventions
                        .eyelids
                        .or(child_entries.try_get_property("eyelids")?);
                }
                NODE_NAME_OBJECT => {
                    let object: String = child_entries.get_argument(0, "name")?;
                    let enabled_value = child_entries.try_get_property("enabled")?;
                    let disabled_value = child_entries.try_get_property("disabled")?;
                    enabled.push(Target::Object {
                        object: object.clone(),
                        value: enabled_value,
                    });
                    disabled.push(Target::Object {
                        object: object.clone(),
                        value: disabled_value,
                    });
                }
                NODE_NAME_SHAPE => {
                    let shape: String = child_entries.get_argument(0, "name")?;
                    let mesh: Option<String> = child_entries.try_get_property("mesh")?;
                    let enabled_value = child_entries.try_get_property("enabled")?;
                    let disabled_value = child_entries.try_get_property("disabled")?;
                    enabled.push(Target::Shape {
                        shape: shape.clone(),
                        mesh: mesh.clone(),
                        value: enabled_value,
                    });
                    disabled.push(Target::Shape {
                        shape: shape.clone(),
                        mesh: mesh.clone(),
                        value: disabled_value,
                    });
                }
                NODE_NAME_MATERIAL => {
                    let slot: i64 = child_entries.get_argument(0, "slot")?;
                    let mesh = child_entries.try_get_property("mesh")?;
                    let enabled_value = child_entries.try_get_property("enabled")?;
                    let disabled_value = child_entries.try_get_property("disabled")?;
                    enabled.push(Target::Material {
                        slot: slot as usize,
                        value: enabled_value,
                        mesh: mesh.clone(),
                    });
                    disabled.push(Target::Material {
                        slot: slot as usize,
                        value: disabled_value,
                        mesh: mesh.clone(),
                    });
                }
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(AnimationSwitch {
            name,
            parameter,
            default_mesh,
            preventions,
            disabled,
            enabled,
        })
    }
}

struct ForPuppet;
impl Compile<(ForPuppet, &KdlNode)> for DeclCompiler {
    type Output = Puppet;

    fn compile(&mut self, (_, node): (ForPuppet, &KdlNode)) -> Result<Puppet> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_PUPPET), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut parameter = None;
        let mut mesh = None;
        let mut keyframes = vec![];

        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, None)?;

            match child_name {
                NODE_NAME_MESH => {
                    mesh = child_entries.try_get_argument(0)?;
                }
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_KEYFRAME => {
                    keyframes.push(self.compile((ForPuppetKeyframe, child))?);
                }
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(Puppet {
            name,
            mesh,
            parameter,
            keyframes,
        })
    }
}

struct ForPuppetKeyframe;
impl Compile<(ForPuppetKeyframe, &KdlNode)> for DeclCompiler {
    type Output = PuppetKeyframe;

    fn compile(&mut self, (_, node): (ForPuppetKeyframe, &KdlNode)) -> Result<PuppetKeyframe> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_KEYFRAME), Some(true))?;
        let position = entries.get_argument(0, "keyframe_position")?;

        let mut targets = vec![];
        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, Some(false))?;
            let target = match child_name {
                NODE_NAME_SHAPE => {
                    let shape = child_entries.get_argument(0, "shape")?;
                    let mesh = child_entries.try_get_property("mesh")?;
                    let value = child_entries.try_get_property("value")?;
                    Target::Shape { shape, mesh, value }
                }
                NODE_NAME_OBJECT => {
                    let object = child_entries.get_argument(0, "object")?;
                    let value = child_entries.try_get_property("value")?;
                    Target::Object { object, value }
                }
                NODE_NAME_MATERIAL => {
                    let slot: i64 = child_entries.get_argument(0, "slot")?;
                    let mesh = child_entries.try_get_property("mesh")?;
                    let value = child_entries.try_get_property("value")?;
                    Target::Material {
                        slot: slot as usize,
                        value,
                        mesh,
                    }
                }
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            };
            targets.push(target);
        }

        Ok(PuppetKeyframe { position, targets })
    }
}

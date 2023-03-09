use crate::{
    compiler::Compile,
    decl::{
        compiler::{deconstruct_node, DeclCompiler},
        data::{
            AnimationElement, Animations, ObjectGroup, ObjectGroupBlock, ObjectSwitch,
            ObjectSwitchPair, ShapeGroup, ShapeGroupBlock, ShapeSwitch, ShapeSwitchPair,
        },
        error::{DeclError, DeclErrorKind, Result},
    },
};

use kdl::KdlNode;

pub const NODE_NAME_ANIMATIONS: &str = "animations";
pub const NODE_NAME_SHAPE_GROUP: &str = "shape-group";
pub const NODE_NAME_SHAPE_SWITCH: &str = "shape-switch";
pub const NODE_NAME_OBJECT_GROUP: &str = "object-group";
pub const NODE_NAME_OBJECT_SWITCH: &str = "object-switch";
pub const NODE_NAME_MESH: &str = "mesh";
pub const NODE_NAME_PARAMETER: &str = "parameter";
pub const NODE_NAME_PREVENT: &str = "prevent";
pub const NODE_NAME_DEFAULT: &str = "default";
pub const NODE_NAME_OPTION: &str = "option";
pub const NODE_NAME_SHAPE: &str = "shape";
pub const NODE_NAME_OBJECT: &str = "object";

pub(super) struct ForAnimations;
impl Compile<(ForAnimations, &KdlNode)> for DeclCompiler {
    type Output = Animations;

    fn compile(&mut self, (_, node): (ForAnimations, &KdlNode)) -> Result<Animations> {
        let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_ANIMATIONS), Some(true))?;

        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SHAPE_GROUP => {
                    AnimationElement::ShapeGroup(self.compile((ForShapeGroup, child))?)
                }
                NODE_NAME_SHAPE_SWITCH => {
                    AnimationElement::ShapeSwitch(self.compile((ForShapeSwitch, child))?)
                }
                NODE_NAME_OBJECT_GROUP => {
                    AnimationElement::ObjectGroup(self.compile((ForObjectGroup, child))?)
                }
                NODE_NAME_OBJECT_SWITCH => {
                    AnimationElement::ObjectSwitch(self.compile((ForObjectSwitch, child))?)
                }
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

struct ForShapeGroup;
impl Compile<(ForShapeGroup, &KdlNode)> for DeclCompiler {
    type Output = ShapeGroup;

    fn compile(&mut self, (_, node): (ForShapeGroup, &KdlNode)) -> Result<ShapeGroup> {
        let (_, entries, children) =
            deconstruct_node(node, Some(NODE_NAME_SHAPE_GROUP), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut mesh = None;
        let mut parameter = None;
        let mut prevent_mouth = None;
        let mut prevent_eyelids = None;
        let mut default_block = None;
        let mut options = vec![];

        let mut option_order = 1;
        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, None)?;

            match child_name {
                NODE_NAME_MESH => {
                    mesh = Some(child_entries.get_argument(0, "mesh")?);
                }
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_PREVENT => {
                    prevent_mouth = child_entries.try_get_property("mouth")?;
                    prevent_eyelids = child_entries.try_get_property("eyelids")?;
                }
                NODE_NAME_DEFAULT => {
                    if default_block.is_some() {
                        return Err(DeclError::new(
                            child.name().span(),
                            DeclErrorKind::DuplicateNodeFound,
                        ));
                    }
                    default_block = Some(self.compile((ForShapeGroupBlock, child, 0))?);
                }
                NODE_NAME_OPTION => {
                    options.push(self.compile((ForShapeGroupBlock, child, option_order))?);
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

        let mesh = mesh.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_MESH),
            )
        })?;
        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(ShapeGroup {
            name,
            mesh,
            parameter,
            prevent_mouth,
            prevent_eyelids,
            default_block,
            options,
        })
    }
}

struct ForShapeGroupBlock;
impl Compile<(ForShapeGroupBlock, &KdlNode, usize)> for DeclCompiler {
    type Output = ShapeGroupBlock;

    fn compile(
        &mut self,
        (_, node, order): (ForShapeGroupBlock, &KdlNode, usize),
    ) -> Result<ShapeGroupBlock> {
        let (name, entries, children) = deconstruct_node(node, None, None)?;

        let mut shapes = vec![];
        let block_name;
        if children.is_empty() {
            if name != NODE_NAME_OPTION {
                return Err(DeclError::new(
                    node.name().span(),
                    DeclErrorKind::InvalidNodeDetected,
                ));
            }
            let option_name: String = entries.get_argument(0, "name")?;
            let shape_name: Option<String> = entries.try_get_property("shape")?;
            let shape_value = entries.try_get_property("value")?;

            block_name = Some(option_name.clone());
            shapes.push((shape_name.unwrap_or(option_name), shape_value));
        } else {
            block_name = match name {
                NODE_NAME_OPTION => Some(entries.get_argument(0, "name")?),
                NODE_NAME_DEFAULT => None,
                _ => unreachable!("block type already refined here"),
            };

            for child in children {
                let (_, child_entries, _) =
                    deconstruct_node(child, Some(NODE_NAME_SHAPE), Some(false))?;

                let shape_name = child_entries.get_argument(0, "shape_name")?;
                let shape_value = child_entries.try_get_property("value")?;
                shapes.push((shape_name, shape_value));
            }
        }

        Ok(ShapeGroupBlock {
            name: block_name,
            declared_order: order,
            shapes,
        })
    }
}

struct ForShapeSwitch;
impl Compile<(ForShapeSwitch, &KdlNode)> for DeclCompiler {
    type Output = ShapeSwitch;

    fn compile(&mut self, (_, node): (ForShapeSwitch, &KdlNode)) -> Result<ShapeSwitch> {
        let (_, entries, children) =
            deconstruct_node(node, Some(NODE_NAME_SHAPE_SWITCH), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut mesh = None;
        let mut parameter = None;
        let mut prevent_mouth = None;
        let mut prevent_eyelids = None;
        let mut shapes = vec![];

        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, Some(false))?;

            match child_name {
                NODE_NAME_MESH => {
                    mesh = Some(child_entries.get_argument(0, "mesh")?);
                }
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_PREVENT => {
                    prevent_mouth = child_entries.try_get_property("mouth")?;
                    prevent_eyelids = child_entries.try_get_property("eyelids")?;
                }
                NODE_NAME_SHAPE => {
                    let shape = child_entries.get_argument(0, "name")?;
                    let enabled = child_entries.try_get_property("enabled")?;
                    let disabled = child_entries.try_get_property("disabled")?;
                    shapes.push(ShapeSwitchPair {
                        shape,
                        disabled,
                        enabled,
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

        let mesh = mesh.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_MESH),
            )
        })?;
        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(ShapeSwitch {
            name,
            mesh,
            parameter,
            prevent_mouth,
            prevent_eyelids,
            shapes,
        })
    }
}

struct ForObjectGroup;
impl Compile<(ForObjectGroup, &KdlNode)> for DeclCompiler {
    type Output = ObjectGroup;

    fn compile(&mut self, (_, node): (ForObjectGroup, &KdlNode)) -> Result<ObjectGroup> {
        let (_, entries, children) =
            deconstruct_node(node, Some(NODE_NAME_OBJECT_GROUP), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut parameter = None;
        let mut default_block = None;
        let mut options = vec![];

        let mut option_order = 1;
        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, None)?;
            match child_name {
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_DEFAULT => {
                    if default_block.is_some() {
                        return Err(DeclError::new(
                            child.name().span(),
                            DeclErrorKind::DuplicateNodeFound,
                        ));
                    }
                    default_block = Some(self.compile((ForObjectGroupBlock, child, 0))?);
                }
                NODE_NAME_OPTION => {
                    options.push(self.compile((ForObjectGroupBlock, child, option_order))?);
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

        Ok(ObjectGroup {
            name,
            parameter,
            default_block,
            options,
        })
    }
}

struct ForObjectGroupBlock;
impl Compile<(ForObjectGroupBlock, &KdlNode, usize)> for DeclCompiler {
    type Output = ObjectGroupBlock;

    fn compile(
        &mut self,
        (_, node, order): (ForObjectGroupBlock, &KdlNode, usize),
    ) -> Result<ObjectGroupBlock> {
        let (name, entries, children) = deconstruct_node(node, None, None)?;

        let mut objects = vec![];
        let block_name;
        if children.is_empty() {
            if name != NODE_NAME_OPTION {
                return Err(DeclError::new(
                    node.name().span(),
                    DeclErrorKind::InvalidNodeDetected,
                ));
            }
            let option_name: String = entries.get_argument(0, "name")?;
            let object_name: Option<String> = entries.try_get_property("shape")?;
            let object_value = entries.try_get_property("value")?;

            block_name = Some(option_name.clone());
            objects.push((object_name.unwrap_or(option_name), object_value));
        } else {
            block_name = match name {
                NODE_NAME_OPTION => Some(entries.get_argument(0, "name")?),
                NODE_NAME_DEFAULT => None,
                _ => unreachable!("block type already refined here"),
            };

            for child in children {
                let (_, child_entries, _) =
                    deconstruct_node(child, Some(NODE_NAME_OBJECT), Some(false))?;

                let object_name = child_entries.get_argument(0, "object_name")?;
                let object_value = child_entries.try_get_property("value")?;
                objects.push((object_name, object_value));
            }
        }

        Ok(ObjectGroupBlock {
            name: block_name,
            declared_order: order,
            objects,
        })
    }
}

struct ForObjectSwitch;
impl Compile<(ForObjectSwitch, &KdlNode)> for DeclCompiler {
    type Output = ObjectSwitch;

    fn compile(&mut self, (_, node): (ForObjectSwitch, &KdlNode)) -> Result<ObjectSwitch> {
        let (_, entries, children) =
            deconstruct_node(node, Some(NODE_NAME_OBJECT_SWITCH), Some(true))?;

        let name = entries.get_argument(0, "name")?;

        let mut parameter = None;
        let mut objects = vec![];

        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(child, None, Some(false))?;

            match child_name {
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_OBJECT => {
                    let shape = child_entries.get_argument(0, "name")?;
                    let enabled = child_entries.try_get_property("enabled")?;
                    let disabled = child_entries.try_get_property("disabled")?;
                    objects.push(ObjectSwitchPair {
                        object: shape,
                        disabled,
                        enabled,
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

        Ok(ObjectSwitch {
            name,
            parameter,
            objects,
        })
    }
}

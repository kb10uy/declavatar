use crate::decl::{deconstruct_node, DeclError, DeclErrorKind, Result};

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

#[derive(Debug, Clone)]
pub struct Animations {
    elements: Vec<AnimationElement>,
}

impl Animations {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, _, children) =
            deconstruct_node(source, node, Some(NODE_NAME_ANIMATIONS), Some(true))?;

        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SHAPE_GROUP => {
                    AnimationElement::ShapeGroup(ShapeGroup::parse(child, source)?)
                }
                NODE_NAME_SHAPE_SWITCH => {
                    AnimationElement::ShapeSwitch(ShapeSwitch::parse(child, source)?)
                }
                NODE_NAME_OBJECT_GROUP => {
                    AnimationElement::ObjectGroup(ObjectGroup::parse(child, source)?)
                }
                NODE_NAME_OBJECT_SWITCH => {
                    AnimationElement::ObjectSwitch(ObjectSwitch::parse(child, source)?)
                }
                _ => {
                    return Err(DeclError::new(
                        source,
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

#[derive(Debug, Clone)]
pub enum AnimationElement {
    ShapeGroup(ShapeGroup),
    ShapeSwitch(ShapeSwitch),
    ObjectGroup(ObjectGroup),
    ObjectSwitch(ObjectSwitch),
}

#[derive(Debug, Clone)]
pub struct ShapeGroup {
    mesh: String,
    parameter: String,
    prevent_mouth: Option<bool>,
    prevent_eyelids: Option<bool>,
    default_block: Option<ShapeGroupBlock>,
    options: Vec<ShapeGroupBlock>,
}

impl ShapeGroup {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, _, children) =
            deconstruct_node(source, node, Some(NODE_NAME_SHAPE_GROUP), Some(true))?;

        let mut mesh = None;
        let mut parameter = None;
        let mut prevent_mouth = None;
        let mut prevent_eyelids = None;
        let mut default_block = None;
        let mut options = vec![];

        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(source, child, None, None)?;

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
                            source,
                            child.name().span(),
                            DeclErrorKind::DuplicateNodeFound,
                        ));
                    }
                    default_block = Some(ShapeGroupBlock::parse(child, source)?);
                }
                NODE_NAME_OPTION => {
                    options.push(ShapeGroupBlock::parse(child, source)?);
                }
                _ => {
                    return Err(DeclError::new(
                        source,
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let mesh = mesh.ok_or_else(|| {
            DeclError::new(
                source,
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_MESH),
            )
        })?;
        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                source,
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(ShapeGroup {
            mesh,
            parameter,
            prevent_mouth,
            prevent_eyelids,
            default_block,
            options,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShapeGroupBlock {
    name: Option<String>,
    shapes: Vec<(String, Option<f64>)>,
}

impl ShapeGroupBlock {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (name, entries, children) = deconstruct_node(source, node, None, Some(true))?;

        let block_name = match name {
            NODE_NAME_OPTION => Some(entries.get_argument(0, "name")?),
            NODE_NAME_DEFAULT => None,
            _ => unreachable!("block type already refined here"),
        };

        let mut shapes = vec![];
        for child in children {
            let (_, child_entries, _) =
                deconstruct_node(source, child, Some(NODE_NAME_SHAPE), Some(false))?;

            let shape_name = child_entries.get_argument(0, "shape_name")?;
            let shape_value = child_entries.try_get_property("value")?;
            shapes.push((shape_name, shape_value));
        }

        Ok(ShapeGroupBlock {
            name: block_name,
            shapes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShapeSwitch {
    mesh: String,
    parameter: String,
    prevent_mouth: Option<bool>,
    prevent_eyelids: Option<bool>,
    shapes: Vec<ShapeSwitchPair>,
}

impl ShapeSwitch {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, entries, children) =
            deconstruct_node(source, node, Some(NODE_NAME_SHAPE_SWITCH), Some(true))?;

        let mut mesh = None;
        let mut parameter = None;
        let mut prevent_mouth = None;
        let mut prevent_eyelids = None;
        let mut shapes = vec![];

        for child in children {
            let (child_name, child_entries, _) =
                deconstruct_node(source, child, None, Some(false))?;

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
                        source,
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let mesh = mesh.ok_or_else(|| {
            DeclError::new(
                source,
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_MESH),
            )
        })?;
        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                source,
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(ShapeSwitch {
            mesh,
            parameter,
            prevent_mouth,
            prevent_eyelids,
            shapes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShapeSwitchPair {
    shape: String,
    enabled: Option<f64>,
    disabled: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ObjectGroup {
    parameter: String,
    default_block: Option<ObjectGroupBlock>,
    options: Vec<ObjectGroupBlock>,
}

impl ObjectGroup {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, _, children) =
            deconstruct_node(source, node, Some(NODE_NAME_OBJECT_GROUP), Some(true))?;

        let mut parameter = None;
        let mut default_block = None;
        let mut options = vec![];

        for child in children {
            let (child_name, child_entries, _) = deconstruct_node(source, child, None, None)?;
            match child_name {
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_DEFAULT => {
                    if default_block.is_some() {
                        return Err(DeclError::new(
                            source,
                            child.name().span(),
                            DeclErrorKind::DuplicateNodeFound,
                        ));
                    }
                    default_block = Some(ObjectGroupBlock::parse(child, source)?);
                }
                NODE_NAME_OPTION => {
                    options.push(ObjectGroupBlock::parse(child, source)?);
                }
                _ => {
                    return Err(DeclError::new(
                        source,
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                source,
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(ObjectGroup {
            parameter,
            default_block,
            options,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ObjectGroupBlock {
    name: Option<String>,
    objects: Vec<(String, Option<bool>)>,
}

impl ObjectGroupBlock {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (name, entries, children) = deconstruct_node(source, node, None, Some(true))?;

        let block_name = match name {
            NODE_NAME_OPTION => Some(entries.get_argument(0, "name")?),
            NODE_NAME_DEFAULT => None,
            _ => unreachable!("block type already refined here"),
        };

        let mut objects = vec![];
        for child in children {
            let (_, child_entries, _) =
                deconstruct_node(source, child, Some(NODE_NAME_OBJECT), Some(false))?;

            let object_name = child_entries.get_argument(0, "object_name")?;
            let object_value = child_entries.try_get_property("value")?;
            objects.push((object_name, object_value));
        }

        Ok(ObjectGroupBlock {
            name: block_name,
            objects,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ObjectSwitch {
    parameter: String,
    objects: Vec<ObjectSwitchPair>,
}

impl ObjectSwitch {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, entries, children) =
            deconstruct_node(source, node, Some(NODE_NAME_OBJECT_SWITCH), Some(true))?;

        let mut parameter = None;
        let mut objects = vec![];

        for child in children {
            let (child_name, child_entries, _) =
                deconstruct_node(source, child, None, Some(false))?;

            match child_name {
                NODE_NAME_PARAMETER => {
                    parameter = Some(child_entries.get_argument(0, "parameter")?);
                }
                NODE_NAME_OBJECT => {
                    let shape = child_entries.get_argument(0, "name")?;
                    let enabled = child_entries.try_get_property("enabled")?;
                    let disabled = child_entries.try_get_property("disabled")?;
                    objects.push(ObjectSwitchPair {
                        shape,
                        disabled,
                        enabled,
                    });
                }
                _ => {
                    return Err(DeclError::new(
                        source,
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            }
        }

        let parameter = parameter.ok_or_else(|| {
            DeclError::new(
                source,
                node.name().span(),
                DeclErrorKind::NodeNotFound(NODE_NAME_PARAMETER),
            )
        })?;

        Ok(ObjectSwitch { parameter, objects })
    }
}

#[derive(Debug, Clone)]
pub struct ObjectSwitchPair {
    shape: String,
    enabled: Option<bool>,
    disabled: Option<bool>,
}

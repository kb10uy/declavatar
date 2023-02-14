use crate::decl::{
    get_argument, split_entries, try_get_property, DeclError, DeclNode, DeclNodeExt, Result,
    VERSION_REQ_SINCE_1_0,
};

use std::collections::HashMap;

use kdl::{KdlNode, KdlValue};
use semver::{Version, VersionReq};

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
pub const NODE_NAME_SHAPE: &str = "option";

#[derive(Debug, Clone)]
pub struct Animations {
    elements: Vec<AnimationElement>,
}

impl DeclNode for Animations {
    const NODE_NAME: &'static str = NODE_NAME_ANIMATIONS;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SHAPE_GROUP => AnimationElement::ShapeGroup(child.parse(version)?),
                NODE_NAME_SHAPE_SWITCH => AnimationElement::ShapeSwitch(child.parse(version)?),
                NODE_NAME_OBJECT_GROUP => AnimationElement::ObjectGroup(child.parse(version)?),
                NODE_NAME_OBJECT_SWITCH => AnimationElement::ObjectSwitch(child.parse(version)?),
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
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

impl DeclNode for ShapeGroup {
    const NODE_NAME: &'static str = NODE_NAME_SHAPE_GROUP;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let mut mesh = None;
        let mut parameter = None;
        let mut prevent_mouth = None;
        let mut prevent_eyelids = None;
        let mut default_block = None;
        let mut options = vec![];

        for child in children {
            let child_name = child.name().value();
            let (child_args, child_props) = split_entries(child.entries());
            match child_name {
                NODE_NAME_MESH => {
                    mesh = Some(get_argument(&child_args, 0, "mesh")?);
                }
                NODE_NAME_PARAMETER => {
                    parameter = Some(get_argument(&child_args, 0, "parameter")?);
                }
                NODE_NAME_PREVENT => {
                    prevent_mouth = try_get_property(&child_props, "mouth")?;
                    prevent_eyelids = try_get_property(&child_props, "eyelids")?;
                }
                NODE_NAME_DEFAULT => {
                    default_block = Some(child.parse_multi(version)?);
                }
                NODE_NAME_OPTION => {
                    options.push(child.parse_multi(version)?);
                }
            }
        }

        let mesh = mesh.ok_or(DeclError::NodeNotFound(
            NODE_NAME_MESH,
            NODE_NAME_SHAPE_GROUP,
        ))?;
        let parameter = parameter.ok_or(DeclError::NodeNotFound(
            NODE_NAME_PARAMETER,
            NODE_NAME_SHAPE_GROUP,
        ))?;

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

impl DeclNode for ShapeGroupBlock {
    const NODE_NAME: &'static str = "";

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let block_name = match name {
            NODE_NAME_OPTION => Some(get_argument(args, 0, "name")?),
            NODE_NAME_DEFAULT => None,
            _ => unreachable!("block type already refined here"),
        };

        let mut shapes = vec![];
        for child in children {
            let child_name = child.name().value();
            if child_name != NODE_NAME_SHAPE {
                return Err(DeclError::InvalidNodeDetected(child_name.into()));
            }

            let (child_args, child_props) = split_entries(child.entries());
            let shape_name = get_argument(&child_args, 0, "shape_name")?;
            let shape_value = try_get_property(&child_props, "value")?;
            shapes.push((shape_name, shape_value));
        }

        Ok(ShapeGroupBlock {
            name: block_name,
            shapes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShapeSwitch {}

impl DeclNode for ShapeSwitch {
    const NODE_NAME: &'static str = NODE_NAME_SHAPE_SWITCH;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ObjectGroup {}

impl DeclNode for ObjectGroup {
    const NODE_NAME: &'static str = NODE_NAME_OBJECT_GROUP;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ObjectSwitch {}

impl DeclNode for ObjectSwitch {
    const NODE_NAME: &'static str = NODE_NAME_OBJECT_SWITCH;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        todo!()
    }
}

use crate::decl::{validate_self_node, DeclError, FromNode, FromNodeExt};

use kdl::KdlNode;

pub const NODE_NAME_ANIMATIONS: &str = "animations";
pub const NODE_NAME_SHAPE_GROUP: &str = "shape-group";
pub const NODE_NAME_SHAPE_SWITCH: &str = "shape-switch";
pub const NODE_NAME_OBJECT_GROUP: &str = "object-group";
pub const NODE_NAME_OBJECT_SWITCH: &str = "object-switch";

/// Animation descriptor. It should has specific structure like below:
#[derive(Debug, Clone)]
pub struct Animations {
    elements: Vec<AnimationElement>,
}

impl FromNode for Animations {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_ANIMATIONS)?;

        let mut elements = vec![];
        let children = node
            .children()
            .ok_or(DeclError::MustHaveChildren(NODE_NAME_ANIMATIONS.into()))?;
        for child in children.nodes() {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SHAPE_GROUP => AnimationElement::ShapeGroup(child.parse()?),
                NODE_NAME_SHAPE_SWITCH => AnimationElement::ShapeSwitch(child.parse()?),
                NODE_NAME_OBJECT_GROUP => AnimationElement::ObjectGroup(child.parse()?),
                NODE_NAME_OBJECT_SWITCH => AnimationElement::ObjectSwitch(child.parse()?),
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
pub struct ShapeGroup {}

impl FromNode for ShapeGroup {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        Ok(ShapeGroup {})
    }
}

#[derive(Debug, Clone)]
pub struct ShapeSwitch {}

impl FromNode for ShapeSwitch {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        Ok(ShapeSwitch {})
    }
}

#[derive(Debug, Clone)]
pub struct ObjectGroup {}

impl FromNode for ObjectGroup {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        Ok(ObjectGroup {})
    }
}

#[derive(Debug, Clone)]
pub struct ObjectSwitch {}

impl FromNode for ObjectSwitch {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        Ok(ObjectSwitch {})
    }
}

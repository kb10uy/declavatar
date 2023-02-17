use crate::decl::{get_argument, try_get_property, DeclError, DeclNode, DeclNodeExt, Result};

use std::collections::HashMap;

use kdl::{KdlNode, KdlValue};
use semver::Version;

use super::get_property;

pub const NODE_NAME_DRIVERS: &str = "drivers";
pub const NODE_NAME_GROUP: &str = "group";
pub const NODE_NAME_SET: &str = "set";
pub const NODE_NAME_ADD: &str = "add";
pub const NODE_NAME_RANDOM: &str = "random";
pub const NODE_NAME_COPY: &str = "copy";

#[derive(Debug, Clone)]
pub struct Drivers {
    groups: Vec<Group>,
}

impl DeclNode for Drivers {
    const NODE_NAME: &'static str = NODE_NAME_DRIVERS;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        _name: &str,
        _args: &[&KdlValue],
        _props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let mut groups = vec![];
        for child in children {
            let child_name = child.name().value();
            let group = match child_name {
                NODE_NAME_GROUP => child.parse(version)?,
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.into())),
            };
            groups.push(group);
        }

        Ok(Drivers { groups })
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    name: String,
    local: Option<bool>,
    drives: Vec<Drive>,
}

impl DeclNode for Group {
    const NODE_NAME: &'static str = NODE_NAME_GROUP;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        _name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let name = get_argument(args, 0, "name")?;
        let local = try_get_property(props, "local")?;
        let mut drives = vec![];

        for child in children {
            let child_name = child.name().value();
            let drive = match child_name {
                NODE_NAME_SET | NODE_NAME_ADD | NODE_NAME_RANDOM | NODE_NAME_COPY => {
                    child.parse_multi(version)?
                }
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.into())),
            };
            drives.push(drive);
        }

        Ok(Group {
            name,
            local,
            drives,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Drive {
    Set(DriveTarget),
    Add(DriveTarget),
    Random {
        group: Option<String>,
        parameter: Option<String>,
        chance: Option<f64>,
        range: (Option<f64>, Option<f64>),
    },
    Copy {
        from: String,
        to: String,
        from_range: (Option<f64>, Option<f64>),
        to_range: (Option<f64>, Option<f64>),
    },
}

impl Drive {
    fn parse_drive_target(props: &HashMap<&str, &KdlValue>) -> Result<DriveTarget> {
        let target_group = try_get_property(props, "group")?;
        let target_parameter = try_get_property(props, "parameter")?;
        let drive_target = match (target_group, target_parameter) {
            (Some(name), None) => {
                let option = try_get_property(props, "option")?;
                DriveTarget::Group { name, option }
            }
            (None, Some(name)) => {
                let value: &KdlValue = get_property(props, "value")?;
                let int_value = value.as_i64();
                let float_value = value.as_f64();
                let bool_value = value.as_bool();
                if let Some(value) = int_value {
                    DriveTarget::IntParameter {
                        name,
                        value: value as u8,
                    }
                } else if let Some(value) = float_value {
                    DriveTarget::FloatParameter { name, value }
                } else if let Some(value) = bool_value {
                    DriveTarget::BoolParameter { name, value }
                } else {
                    return Err(DeclError::IncorrectType("int, float, or bool"));
                }
            }
            _ => {
                return Err(DeclError::InvalidNodeDetected(
                    "ambiguous driver target".into(),
                ));
            }
        };

        Ok(drive_target)
    }
}

impl DeclNode for Drive {
    const NODE_NAME: &'static str = "";

    const CHILDREN_EXISTENCE: Option<bool> = Some(false);

    fn parse(
        _version: &Version,
        name: &str,
        _args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        _children: &[KdlNode],
    ) -> Result<Self> {
        let drive = match name {
            NODE_NAME_SET => {
                let drive_target = Drive::parse_drive_target(props)?;
                Drive::Set(drive_target)
            }
            NODE_NAME_ADD => {
                // Just reuses DriveTarget.
                // Only Integer/FloatParameter affects, verifies that following step.
                let drive_target = Drive::parse_drive_target(props)?;
                Drive::Set(drive_target)
            }
            NODE_NAME_RANDOM => {
                let group = try_get_property(props, "group")?;
                let parameter = try_get_property(props, "parameter")?;
                let range_min = try_get_property(props, "min")?;
                let range_max = try_get_property(props, "max")?;
                let chance = try_get_property(props, "chance")?;
                Drive::Random {
                    group,
                    parameter,
                    chance,
                    range: (range_min, range_max),
                }
            }
            NODE_NAME_COPY => {
                let from = get_property(props, "from")?;
                let to = get_property(props, "from")?;
                let from_min = try_get_property(props, "from_min")?;
                let from_max = try_get_property(props, "from_max")?;
                let to_min = try_get_property(props, "to_min")?;
                let to_max = try_get_property(props, "to_max")?;
                Drive::Copy {
                    from,
                    to,
                    from_range: (from_min, from_max),
                    to_range: (to_min, to_max),
                }
            }
            _ => unreachable!("drive type already refined here"),
        };
        Ok(drive)
    }
}

#[derive(Debug, Clone)]
pub enum DriveTarget {
    Group { name: String, option: Option<String> },
    IntParameter { name: String, value: u8 },
    FloatParameter { name: String, value: f64 },
    BoolParameter { name: String, value: bool },
}

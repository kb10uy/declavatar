use crate::{
    compiler::Compile,
    decl::{
        compiler::{deconstruct_node, DeclCompiler, DeclError, DeclErrorKind, NodeEntries, Result},
        data::{Drive, DriveTarget, DriverGroup, Drivers},
    },
};

use kdl::{KdlNode, KdlValue};

pub const NODE_NAME_DRIVERS: &str = "drivers";
pub const NODE_NAME_GROUP: &str = "group";
pub const NODE_NAME_SET: &str = "set";
pub const NODE_NAME_ADD: &str = "add";
pub const NODE_NAME_RANDOM: &str = "random";
pub const NODE_NAME_COPY: &str = "copy";

pub(super) struct ForDrivers;
impl Compile<(ForDrivers, &KdlNode)> for DeclCompiler {
    type Output = Drivers;

    fn compile(&mut self, (_, node): (ForDrivers, &KdlNode)) -> Result<Drivers> {
        let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_DRIVERS), Some(true))?;

        let mut groups = vec![];
        for child in children {
            let child_name = child.name().value();
            let group = match child_name {
                NODE_NAME_GROUP => self.compile((ForDriverGroup, child))?,
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            };
            groups.push(group);
        }

        Ok(Drivers { groups })
    }
}

struct ForDriverGroup;
impl Compile<(ForDriverGroup, &KdlNode)> for DeclCompiler {
    type Output = DriverGroup;

    fn compile(&mut self, (_, node): (ForDriverGroup, &KdlNode)) -> Result<DriverGroup> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_GROUP), Some(true))?;

        let name = entries.get_argument(0, "name")?;
        let local = entries.try_get_property("local")?;
        let mut drives = vec![];

        for child in children {
            let child_name = child.name().value();
            let drive = match child_name {
                NODE_NAME_SET | NODE_NAME_ADD | NODE_NAME_RANDOM | NODE_NAME_COPY => {
                    self.compile((ForDrive, child))?
                }
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            };
            drives.push(drive);
        }

        Ok(DriverGroup {
            name,
            local,
            drives,
        })
    }
}

struct ForDrive;
impl Compile<(ForDrive, &KdlNode)> for DeclCompiler {
    type Output = Drive;

    fn compile(&mut self, (_, node): (ForDrive, &KdlNode)) -> Result<Drive> {
        let (name, entries, _) = deconstruct_node(node, None, Some(false))?;

        let drive = match name {
            NODE_NAME_SET => {
                let drive_target = parse_drive_target(&entries, node)?;
                Drive::Set(drive_target)
            }
            NODE_NAME_ADD => {
                // Just reuses DriveTarget.
                // Only Integer/FloatParameter affects, verifies that following step.
                let drive_target = parse_drive_target(&entries, node)?;
                Drive::Add(drive_target)
            }
            NODE_NAME_RANDOM => {
                let group = entries.try_get_property("group")?;
                let parameter = entries.try_get_property("parameter")?;
                let range_min = entries.try_get_property("min")?;
                let range_max = entries.try_get_property("max")?;
                let chance = entries.try_get_property("chance")?;
                Drive::Random {
                    group,
                    parameter,
                    chance,
                    range: (range_min, range_max),
                }
            }
            NODE_NAME_COPY => {
                let from = entries.get_property("from")?;
                let to = entries.get_property("from")?;
                let from_min = entries.try_get_property("from_min")?;
                let from_max = entries.try_get_property("from_max")?;
                let to_min = entries.try_get_property("to_min")?;
                let to_max = entries.try_get_property("to_max")?;
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

fn parse_drive_target(entries: &NodeEntries, parent: &KdlNode) -> Result<DriveTarget> {
    let target_group = entries.try_get_property("group")?;
    let target_parameter = entries.try_get_property("parameter")?;
    let drive_target = match (target_group, target_parameter) {
        (Some(name), None) => {
            let option = entries.try_get_property("option")?;
            DriveTarget::Group { name, option }
        }
        (None, Some(name)) => {
            let value: &KdlValue = entries.get_property("value")?;
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
                let entry_span = parent.get("value").expect("must have entry").span();
                return Err(DeclError::new(
                    entry_span,
                    DeclErrorKind::IncorrectType("int or bool"),
                ));
            }
        }
        _ => {
            return Err(DeclError::new(
                parent.name().span(),
                DeclErrorKind::InvalidNodeDetected,
            ));
        }
    };

    Ok(drive_target)
}

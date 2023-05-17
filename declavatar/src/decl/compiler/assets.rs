use crate::{
    compiler::Compile,
    decl::{
        compiler::{deconstruct_node, DeclCompiler},
        data::{AssetKey, AssetType, Assets},
        error::{DeclError, DeclErrorKind, Result},
    },
};

use kdl::KdlNode;

pub const NODE_NAME_ASSETS: &str = "assets";
const NODE_NAME_MATERIAL: &str = "material";
const NODE_NAME_ANIMATION: &str = "animation";

pub(super) struct ForAssets;
impl Compile<(ForAssets, &KdlNode)> for DeclCompiler {
    type Output = Assets;

    fn compile(&mut self, (_, node): (ForAssets, &KdlNode)) -> Result<Assets> {
        let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_ASSETS), Some(true))?;

        let mut assets = vec![];
        for child in children {
            let child_name = child.name().value();
            let asset_key = match child_name {
                NODE_NAME_MATERIAL => self.compile((ForAsset, AssetType::Material, child))?,
                NODE_NAME_ANIMATION => self.compile((ForAsset, AssetType::Animation, child))?,
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::MustHaveChildren,
                    ));
                }
            };
            assets.push(asset_key);
        }

        Ok(Assets { assets })
    }
}

struct ForAsset;
impl Compile<(ForAsset, AssetType, &KdlNode)> for DeclCompiler {
    type Output = AssetKey;

    fn compile(&mut self, (_, ty, node): (ForAsset, AssetType, &KdlNode)) -> Result<AssetKey> {
        let (_, entries, _) = deconstruct_node(node, None, Some(false))?;
        let key = entries.get_argument(0, "key")?;
        Ok(AssetKey { ty, key })
    }
}

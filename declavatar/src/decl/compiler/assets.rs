use crate::decl::{
    compiler::deconstruct_node,
    data::{AssetKey, AssetType, Assets},
    error::{DeclError, DeclErrorKind, Result},
};

use kdl::KdlNode;

pub const NODE_NAME_ASSETS: &str = "assets";
const NODE_NAME_MATERIAL: &str = "material";
const NODE_NAME_ANIMATION: &str = "animation";

pub fn compile_assets(node: &KdlNode) -> Result<Assets> {
    let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_ASSETS), Some(true))?;

    let mut assets = vec![];
    for child in children {
        let child_name = child.name().value();
        let asset_key = match child_name {
            NODE_NAME_MATERIAL => compile_asset(AssetType::Material, child)?,
            NODE_NAME_ANIMATION => compile_asset(AssetType::Animation, child)?,
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

fn compile_asset(ty: AssetType, node: &KdlNode) -> Result<AssetKey> {
    let (_, entries, _) = deconstruct_node(node, None, Some(false))?;
    let key = entries.get_argument(0, "key")?;
    Ok(AssetKey { ty, key })
}

#[cfg(test)]
mod test {
    use crate::{
        decl::data::{AssetKey, AssetType},
        testing::parse_node,
    };

    use super::compile_assets;

    #[test]
    fn assets_block_compiles() {
        let block_doc = parse_node(
            r#"
            assets {
                material "foo"
                animation "bar"
            }
            "#,
        );
        let block_node = &block_doc.nodes()[0];

        let block = compile_assets(block_node).expect("failed to compile parameters block");
        assert_eq!(block.assets.len(), 2);
        assert_eq!(
            block.assets[0],
            AssetKey {
                key: "foo".to_string(),
                ty: AssetType::Material,
            }
        );
        assert_eq!(
            block.assets[1],
            AssetKey {
                key: "bar".to_string(),
                ty: AssetType::Animation,
            }
        );
    }
}

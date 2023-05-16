use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{Asset, AssetType},
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{AssetType as DeclAssetType, Assets as DeclAssets},
};

impl Compile<Vec<DeclAssets>> for AvatarCompiler {
    type Output = Vec<Asset>;

    fn compile(&mut self, asset_blocks: Vec<DeclAssets>) -> Result<Vec<Asset>> {
        let mut assets: Vec<Asset> = vec![];

        let decl_assets = asset_blocks.into_iter().flat_map(|ab| ab.assets);
        for decl_asset in decl_assets {
            let key = decl_asset.key;
            let asset_type = match decl_asset.ty {
                DeclAssetType::Material => AssetType::Material,
                DeclAssetType::Animation => AssetType::Animation,
                DeclAssetType::Indeterminate => {
                    self.error(format!("indeterminate asset {key} detected"));
                    continue;
                }
            };

            if let Some(defined) = assets.iter().find(|a| a.key == key) {
                if defined.asset_type != asset_type {
                    self.error(format!("asset '{key}' have incompatible declarations"));
                }
                continue;
            }

            assets.push(Asset { asset_type, key });
        }

        Ok(assets)
    }
}

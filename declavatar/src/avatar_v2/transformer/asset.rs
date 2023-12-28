use crate::{
    avatar_v2::{
        data::asset::{Asset, AssetType},
        log::Log,
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::asset::{DeclAsset, DeclAssets},
    log::Logger,
};

pub fn compile_assets_blocks(
    logger: &Logger<Log>,
    assets_blocks: Vec<DeclAssets>,
) -> Compiled<Vec<Asset>> {
    let mut assets = vec![];
    for (index, decl_assets) in assets_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("assets block {index}"));
        for decl_asset in decl_assets.assets {
            let Some(asset) = compile_asset(&logger, decl_asset, &assets) else {
                continue;
            };
            assets.push(asset);
        }
    }

    success(assets)
}

fn compile_asset(
    logger: &Logger<Log>,
    decl_asset: DeclAsset,
    declared: &[Asset],
) -> Compiled<Asset> {
    let key = match &decl_asset {
        DeclAsset::Material(key) => key,
        DeclAsset::Animation(key) => key,
    };
    let asset_type = match &decl_asset {
        DeclAsset::Material(_) => AssetType::Material,
        DeclAsset::Animation(_) => AssetType::Animation,
    };

    if let Some(defined) = declared.iter().find(|a| a.key == *key) {
        if defined.asset_type != asset_type {
            logger.log(Log::IncompatibleAssetDeclaration(key.to_string()));
        }
        return failure();
    }

    success(Asset {
        asset_type,
        key: key.to_string(),
    })
}

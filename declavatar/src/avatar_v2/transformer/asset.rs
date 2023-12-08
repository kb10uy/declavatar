use crate::{
    avatar_v2::{
        data::asset::{Asset, AssetType},
        logging::{LogKind, LoggingContext},
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::asset::{DeclAsset, DeclAssets},
};

pub fn compile_assets_blocks(
    ctx: &mut LoggingContext,
    assets_blocks: Vec<DeclAssets>,
) -> Compiled<Vec<Asset>> {
    let mut assets: Vec<Asset> = vec![];

    let decl_assets = assets_blocks.into_iter().flat_map(|ab| ab.assets);
    for decl_asset in decl_assets {
        let Some(asset) = compile_asset(ctx, decl_asset, &assets) else {
            continue;
        };
        assets.push(asset);
    }

    success(assets)
}

fn compile_asset(
    ctx: &mut LoggingContext,
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
            ctx.log_error(LogKind::IncompatibleAssetDeclaration(key.to_string()));
        }
        return failure();
    }

    success(Asset {
        asset_type,
        key: key.to_string(),
    })
}
